use crate::{
    DATA_FOLDER,
    db::{Singleton, db},
    steam::Steam,
};
use anyhow::{bail, ensure};
use constcat::concat;
use derive_more::Display;
use eventador::{Eventador, Publisher};
use server_z_macros::{arch_alt, github_release_asset_bytes};
use std::{
    env::consts::OS,
    io::{ErrorKind, Read, Write},
    path::Path,
};
use stream_download::{StreamDownload, storage::temp::TempStorageProvider};
use tempfile::NamedTempFile;
use tokio::{fs::create_dir_all, task::spawn_blocking};
use topcoat::context::{Cx, memoize};
use tracing::instrument;
use zip::ZipArchive;

#[derive(Debug, Clone, Copy, Display)]
pub enum DownloadEvent {
    #[display("Downloading '{name}', ({current} out of {total} bytes)")]
    Downloading { name: &'static str, is_needing_extract: bool, current: u128, total: u128 },
    #[display("Downloading '{name}', ({current} out of {total} bytes)")]
    Extracting { name: &'static str, current: u128, total: u128 },
    #[display("Downloaded '{name}' ({total} bytes)")]
    Completed { name: &'static str, total: u128 },
}

pub struct Depot {}

/// Associated functions & constants
impl Depot {
    const BINARY_FILENAME: &str = "DepotDownloader";
    const BINARY_FILEPATH: &str = concat!(DATA_FOLDER, "/bin/", Depot::BINARY_FILENAME);

    const ZIP_FILENAME: &str = concat!("DepotDownloader-", OS, "-", arch_alt!(), ".zip");
    const ZIP_FILEPATH: &str = concat!(DATA_FOLDER, "/tmp/", Depot::ZIP_FILENAME);

    const DOWNLOAD_ZIP_BYTES: u128 = github_release_asset_bytes!(
        "https://github.com/SteamRE/DepotDownloader/releases/tag/DepotDownloader_3.4.0",
        "DepotDownloader-{OS}-{ARCH_ALT}.zip"
    );
    const DOWNLOAD_BASE_URL: &str =
        "https://github.com/SteamRE/DepotDownloader/releases/download/DepotDownloader_3.4.0";
    const DOWNLOAD_ZIP_URL: &str = concat!(Depot::DOWNLOAD_BASE_URL, "/", Depot::ZIP_FILENAME);

    pub async fn is_downloaded() -> anyhow::Result<bool> {
        std::fs::exists(Depot::BINARY_FILEPATH).map_err(Into::into)
    }

    pub async fn download(pub_progress: Option<&Eventador>) -> anyhow::Result<()> {
        let download_publisher = pub_progress.as_ref().map(|v| v.publisher());
        let zip_file = Depot::download_zip(download_publisher).await?;

        let archive = zip::ZipArchive::new(zip_file)?;
        let extract_publisher = pub_progress.as_ref().map(|v| v.publisher());
        Depot::extract(archive, Depot::BINARY_FILENAME, Depot::BINARY_FILEPATH, extract_publisher).await
    }

    // pub async fn download_and_subscribe() -> anyhow::Result<()> {
    //     // Depot::download().
    // }

    async fn extract<P>(
        mut archive: ZipArchive<NamedTempFile>,
        filename: &'static str,
        to: P,
        mut pub_progress: Option<Publisher>,
    ) -> anyhow::Result<()>
    where
        P: AsRef<Path> + Send + 'static,
    {
        spawn_blocking(move || {
            let mut file = match archive.by_name(filename) {
                Ok(file) => file,
                Err(e) => {
                    println!("Depot binary not found in download (zip)");
                    return Err(e.into());
                }
            };

            let total = file.size() as u128;
            let current: u128 = 0;

            let mut tmp_file = NamedTempFile::new_in(to.as_ref())?;

            loop {
                let mut buf = [0u8; 1024];
                let bytes_read = file.read(&mut buf)?;

                if let Some(publisher) = pub_progress.as_mut() {
                    let event = match bytes_read {
                        0 => DownloadEvent::Completed { name: Depot::BINARY_FILENAME, total },
                        _ => DownloadEvent::Extracting { name: Depot::BINARY_FILENAME, current, total },
                    };

                    publisher.send(event);
                }

                if bytes_read == 0 {
                    break;
                }

                tmp_file.write_all(&buf[0..bytes_read])?;
            }

            tmp_file.persist(to)?;
            Ok::<(), anyhow::Error>(())
        })
        .await?
    }

    async fn download_zip(mut pub_progress: Option<Publisher>) -> anyhow::Result<NamedTempFile> {
        let mut reader = match StreamDownload::new_http(
            Depot::DOWNLOAD_ZIP_URL.parse()?,
            TempStorageProvider::new(),
            stream_download::Settings::default(),
        )
        .await
        {
            Ok(reader) => reader,
            Err(err) => {
                bail!("Failed to start download of depot bin: {err:?}");
            }
        };

        let binary_filepath = Path::new(Depot::ZIP_FILEPATH);
        if let Some(parent) = binary_filepath.parent() {
            create_dir_all(parent).await?;
        }

        let download_file = spawn_blocking(move || {
            let mut download_file = NamedTempFile::new_in(binary_filepath)?;
            let mut current: u128 = 0;
            loop {
                let mut buf = [0u8; 1024];
                let bytes_read = reader.read(&mut buf)?;
                current += bytes_read as u128;

                if let Some(publisher) = pub_progress.as_mut() {
                    let event = match bytes_read {
                        0 => {
                            DownloadEvent::Completed { name: Depot::BINARY_FILENAME, total: Depot::DOWNLOAD_ZIP_BYTES }
                        }
                        _ => DownloadEvent::Downloading {
                            name: Depot::BINARY_FILENAME,
                            is_needing_extract: true,
                            current,
                            total: Depot::DOWNLOAD_ZIP_BYTES,
                        },
                    };

                    publisher.send(event);
                }

                if bytes_read == 0 {
                    break;
                }

                download_file.write_all(&buf[0..bytes_read])?;
            }
            Ok::<NamedTempFile, std::io::Error>(download_file)
        })
        .await??;

        if let Err(err) = tokio::fs::remove_file(binary_filepath).await
            && err.kind() != ErrorKind::NotFound
        {
            bail!("Failed to remove old depot binary: {err:?}");
        }

        Ok(download_file)
    }
}

#[memoize(as_ref)]
#[instrument(level = "debug", skip_all, ret)]
pub async fn ensure_steam_is_setup(cx: &Cx) -> anyhow::Result<()> {
    let is_created = Steam::fetch(cx).await.is_ok();
    ensure!(is_created, "Steam is not setup");
    ensure!(Depot::is_downloaded().await?, "Steam DepotDownloader is not downloaded");

    Ok(())
}

#[memoize(as_ref)]
#[instrument(level = "debug", skip_all, ret)]
pub async fn ensure_steam_is_not_setup(cx: &Cx) -> anyhow::Result<()> {
    let is_not_created = Steam::fetch(cx).await.is_err();
    ensure!(is_not_created, "Steam is already setup");
    ensure!(!Depot::is_downloaded().await?, "Steam DepotDownloader is already downloaded");

    Ok(())
}
