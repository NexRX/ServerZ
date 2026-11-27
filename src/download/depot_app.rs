use crate::download::{DownloadEvent, Downloader};
use anyhow::{Result, anyhow, bail};
use std::{process::Stdio, sync::Arc};
use tokio::{
    fs::{canonicalize, create_dir_all},
    io::{AsyncBufReadExt as _, BufReader, Lines},
    process::{Child, ChildStdout, Command},
    sync::broadcast::{Receiver, Sender},
};
use tracing::instrument;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct DepotAppDownload {
    app_id: u32,
    #[builder(setter(into))]
    steam_username: String,
    #[builder(setter(into))]
    current_dir: String,
    #[builder(default_code = "tokio::sync::broadcast::channel(crate::STD_EVENT_BUFFER).0", setter(skip))]
    eventbus: Sender<DownloadEvent>,
}

// Methods
impl DepotAppDownload {
    #[instrument(level = "debug", skip_all, err)]
    async fn download_app(&self) -> Result<()> {
        let (mut child, mut lines) = self.spawn_app_dl(false).await?;
        let name = Arc::from(format!("App ID {}", self.app_id));

        while let Some(line) = lines.next_line().await? {
            tracing::trace!(?line, "New line");

            // e.g. " 99.12% depots/223354/26220720/mpmissions/dayzOffline.sakhal/mapgroupdirt.xml"
            if line.contains("% depot") {
                let mut split = line.trim_start().split(' ');
                let current: f32 = split
                    .next()
                    .ok_or_else(|| anyhow!("no progress in line"))?
                    .replace("%", "")
                    .parse()
                    .map_err(|_| anyhow!("progress couldn't be parsed"))?;
                let current_file = Arc::from(split.next().ok_or(anyhow!("no file in line"))?.to_owned());
                self.eventbus.send(DownloadEvent::Downloading {
                    name: Arc::clone(&name),
                    current_file,
                    current,
                    total: 100f32,
                    has_extract_step: false,
                })?;
            }

            // e.g. "Total downloaded: 2284736752 bytes (5790734490 bytes uncompressed) from 2 depots"
            if line.starts_with("Total downloaded:") {
                let mut split = line.split(' ');
                let total: f32 = split
                    .nth(3)
                    .ok_or_else(|| anyhow!("no bytes in line part (a)"))?
                    .parse()
                    .map_err(|_| anyhow!("bytes couldn't be parsed (a)"))?;
                // let total_uncompressed: u64 = split.nth(5).ok_or_else(|| anyhow!("no bytes in line part (b)"))?.replace("(", "").parse().map_err(|_| anyhow!("bytes couldn't be parsed (b)"))?;
                let name = format!("App ID {}", self.app_id);
                self.eventbus.send(DownloadEvent::Completed { name: Arc::from(name), total })?;
            }
        }

        if !child.wait().await?.success() {
            const MESSAGE: &str = "Download completed depot exited unsuccessfully";
            self.eventbus.send(DownloadEvent::Failed { name, message: MESSAGE.to_string() })?;
            bail!("Failed depot download: {MESSAGE}");
        }
        Ok(())
    }

    #[instrument(level = "trace", skip_all, err)]
    async fn spawn_app_dl(&self, manifest_only: bool) -> Result<(Child, Lines<BufReader<ChildStdout>>)> {
        create_dir_all(&self.current_dir).await?;
        let path = canonicalize(crate::steam::Depot::BINARY_FILEPATH).await?;
        let mut child = Command::new(path)
            .current_dir(&self.current_dir)
            .arg("-app")
            .arg(self.app_id.to_string())
            .arg("-username")
            .arg(&self.steam_username)
            .arg("-remember-password")
            .arg(if manifest_only { "-manifest-only" } else { "" })
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed depot sign-in (internal error 0b)"))?;
        let lines = BufReader::new(stdout).lines();
        Ok((child, lines))
    }
}

impl Downloader for DepotAppDownload {
    fn subscribe(&self) -> Receiver<DownloadEvent> {
        self.eventbus.subscribe()
    }

    fn start_bg(&self) {
        let depot = self.clone();
        tokio::spawn(async move { depot.download_app().await });
    }

    fn has_extract_step(&self) -> bool {
        false
    }

    fn name(&self) -> String {
        format!("App ID: {}", self.app_id)
    }
}
