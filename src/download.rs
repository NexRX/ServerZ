use crate::TMP_FOLDER;
use anyhow::{Result, bail};
use derive_more::{Deref, DerefMut, Display, IsVariant};
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;
use std::{fmt::Debug, fs::File};
use std::{
    io::{ErrorKind, Read, Write},
    path::Path,
};
use stream_download::{StreamDownload, storage::temp::TempStorageProvider};
use tempfile::NamedTempFile;
use tokio::fs;
use tokio::sync::broadcast::error::{RecvError, TryRecvError};
use tokio::{
    fs::create_dir_all,
    sync::broadcast::{self, Receiver, Sender},
    task::spawn_blocking,
};
use tracing::{debug, info, instrument};
use typed_builder::TypedBuilder;
use zip::ZipArchive;

#[derive(TypedBuilder)]
pub struct Downloader {
    #[builder(setter(into))]
    url: String,
    #[builder(setter(into))]
    target: String,
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    total_download_bytes: u128,
    #[builder(default, setter(strip_option, into))]
    filename_in_archive: Option<String>,
    #[builder(default = true, setter(into))]
    remove_target: bool,
    #[builder(default = false, setter(into))]
    make_executable: bool,

    #[builder(default, setter(skip))]
    eventbus: Option<Sender<DownloadEvent>>,
}

#[derive(Debug, Clone, Display, IsVariant)]
pub enum DownloadEvent {
    #[display("Downloading '{name}', ({current} out of {total} bytes)")]
    Downloading { name: Arc<str>, has_extract_step: bool, current: u128, total: u128 },
    #[display("Extracting '{name}', ({current} out of {total} bytes)")]
    Extracting { name: Arc<str>, current: u128, total: u128 },
    #[display("Completed '{name}' ({total} bytes)")]
    Completed { name: Arc<str>, total: u128 },
}

#[derive(Deref, DerefMut)]
struct Publisher {
    name: Arc<str>,
    has_extract_step: bool,
    total_download_bytes: u128,
    #[deref]
    #[deref_mut]
    inner: Sender<DownloadEvent>,
}

/// Methods
impl Downloader {
    pub fn has_extract_step(&self) -> bool {
        self.filename_in_archive.is_some()
    }

    pub fn subscribe(&mut self) -> Receiver<DownloadEvent> {
        self.eventbus.get_or_insert_with(|| broadcast::channel(crate::STD_EVENT_BUFFER).0).subscribe()
    }

    pub fn start_bg(self) {
        tokio::spawn(self.start());
    }

    #[instrument(level = "info", name = "Download::start", skip_all, fields(name = %self.name, url = %self.url), ret, err)]
    pub async fn start(self) -> Result<File> {
        info!("Starting");
        match self.filename_in_archive.as_ref() {
            Some(filename_in_archive) => self.start_with_extract(filename_in_archive).await,
            None => self.start_without_extract().await,
        }
    }

    async fn start_with_extract(&self, filename_in_archive: &str) -> Result<File> {
        let tmp_zip_file = download(&self.url, self.publisher()).await?;

        let archive = zip::ZipArchive::new(tmp_zip_file)?;
        let tmp_file = extract(archive, filename_in_archive, &self.target, self.publisher()).await?;

        self.persist(tmp_file).await
    }

    async fn start_without_extract(&self) -> Result<File> {
        let tmp_file = download(&self.url, self.publisher()).await?;
        self.persist(tmp_file).await
    }

    async fn persist(&self, tmp_file: NamedTempFile) -> Result<File> {
        if self.remove_target {
            remove_file(&self.target).await?;
        }
        if let Some(parent) = Path::new(&self.target).parent() {
            create_dir_all(parent).await?;
        }
        let file = tmp_file.persist(&self.target)?;
        if self.make_executable {
            make_executable(&self.target).await?;
        }

        Ok(file)
    }

    fn publisher(&self) -> Option<Publisher> {
        self.eventbus.as_ref().map(|eventbus| Publisher {
            name: Arc::from(self.name.to_owned()),
            has_extract_step: self.filename_in_archive.is_some(),
            total_download_bytes: self.total_download_bytes,
            inner: eventbus.clone(),
        })
    }
}

pub trait ReceiveLatest<T: Clone> {
    fn receiver(&mut self) -> &mut Receiver<T>;

    async fn recv_latest(&mut self) -> Option<T> {
        let receiver = self.receiver();

        let mut latest = loop {
            match receiver.recv().await {
                Ok(event) => break event,
                Err(RecvError::Lagged(_)) => continue,
                Err(RecvError::Closed) => return None,
            }
        };

        loop {
            match receiver.try_recv() {
                Ok(event) => latest = event,
                Err(TryRecvError::Lagged(_)) => continue,
                Err(TryRecvError::Empty | TryRecvError::Closed) => return Some(latest),
            }
        }
    }
}

impl<T: Clone> ReceiveLatest<T> for Receiver<T> {
    fn receiver(&mut self) -> &mut Receiver<T> {
        self
    }
}

#[instrument(level = "debug", skip(pub_progress), ret, err)]
async fn extract(
    mut archive: ZipArchive<NamedTempFile>,
    filename_in_archive: impl ToString + Debug,
    to: impl ToString + Debug,
    mut pub_progress: Option<Publisher>,
) -> Result<NamedTempFile> {
    let filename_in_archive = filename_in_archive.to_string();
    let to = to.to_string();

    let parent = Path::new(&to).parent().unwrap_or(Path::new(&to));
    create_dir_all(parent).await?;

    spawn_blocking(move || {
        debug!("Starting extract");
        let mut file = match archive.by_name(&filename_in_archive) {
            Ok(file) => file,
            Err(e) => {
                println!("File not found in archive");
                return Err(e.into());
            }
        };

        let total = file.size() as u128;
        let mut current: u128 = 0;

        let mut tmp_file = NamedTempFile::new_in(TMP_FOLDER)?;

        loop {
            let mut buf = [0u8; 1024];
            let bytes_read = file.read(&mut buf)?;
            current += bytes_read as u128;

            if let Some(p) = pub_progress.as_mut() {
                let event = match bytes_read {
                    0 => DownloadEvent::Completed { name: Arc::clone(&p.name), total },
                    _ => DownloadEvent::Extracting { name: Arc::clone(&p.name), current, total },
                };
                let _ = p.send(event);
            }

            if bytes_read == 0 {
                break;
            }

            tmp_file.write_all(&buf[0..bytes_read])?;
        }

        Ok::<NamedTempFile, anyhow::Error>(tmp_file)
    })
    .await?
}

#[instrument(level = "debug", skip_all, ret, err)]
async fn download(url: impl ToString + Debug, mut pub_progress: Option<Publisher>) -> Result<NamedTempFile> {
    let url = url.to_string();
    let mut reader =
        match StreamDownload::new_http(url.parse()?, TempStorageProvider::new(), stream_download::Settings::default())
            .await
        {
            Ok(reader) => reader,
            Err(err) => {
                bail!("Failed to start download: {err:?}");
            }
        };

    let download_file = spawn_blocking(move || {
        let mut tmp_file = NamedTempFile::new_in(TMP_FOLDER)?;
        let mut current: u128 = 0;
        loop {
            let mut buf = [0u8; 1024];
            let bytes_read = reader.read(&mut buf)?;
            current += bytes_read as u128;

            if let Some(p) = pub_progress.as_mut() {
                let event = match bytes_read {
                    0 if !p.has_extract_step => {
                        DownloadEvent::Completed { name: Arc::clone(&p.name), total: p.total_download_bytes }
                    }
                    _ => DownloadEvent::Downloading {
                        name: Arc::clone(&p.name),
                        has_extract_step: p.has_extract_step,
                        current,
                        total: p.total_download_bytes,
                    },
                };
                let _ = p.send(event);
            }

            if bytes_read == 0 {
                break;
            }

            tmp_file.write_all(&buf[0..bytes_read])?;
        }
        Ok::<NamedTempFile, std::io::Error>(tmp_file)
    })
    .await??;

    Ok(download_file)
}

#[instrument(level = "trace", skip_all, ret, err)]
pub async fn make_executable(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let mut perms = fs::metadata(path).await?.permissions();
    let current_mode = perms.mode();
    perms.set_mode(current_mode | 0o110); // Adds +x for user, group, and others
    fs::set_permissions(path, perms).await?;
    Ok(())
}

#[instrument(level = "trace", skip_all, ret, err)]
pub async fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    if let Err(err) = tokio::fs::remove_file(path).await
        && err.kind() != ErrorKind::NotFound
    {
        bail!("Failed to remove file: {err:?}");
    }
    Ok(())
}
