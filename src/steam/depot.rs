use crate::{DATA_FOLDER, download::HttpDownloader, steam::DepotAuthenticator};
use anyhow::{Result, anyhow, bail};
use constcat::concat;
use server_z_macros::{arch_alt, github_release_asset_bytes};
use std::{env::consts::OS, process::Stdio};
use tokio::{
    fs::{canonicalize, create_dir_all},
    io::{AsyncBufReadExt as _, BufReader, Lines},
    process::{Child, ChildStdout, Command},
    sync::broadcast::{self, Receiver, Sender},
};
use tracing::instrument;

#[derive(Debug, Clone)]
pub enum DepotDownloadEvent {
    Failed { message: String },
    Progress { current: f32, file: String },
    Completed { total: u64, total_uncompressed: u64 },
}

#[derive(Debug, Clone)]
pub struct Depot {
    steam_username: String,
    current_dir: String,
    eventbus: Sender<DepotDownloadEvent>,
}

// Methods
impl Depot {
    pub fn subscribe(&self) -> Receiver<DepotDownloadEvent> {
        self.eventbus.subscribe()
    }

    pub fn download_app(&self, app_id: u32) -> tokio::task::JoinHandle<Result<()>> {
        let depot = self.clone();
        tokio::spawn(async move { depot.download_app_impl(app_id).await })
    }

    #[instrument(level = "debug", skip_all, err)]
    async fn download_app_impl(&self, app_id: u32) -> Result<()> {
        let (mut child, mut lines) = self.spawn_app_dl(app_id, false).await?;

        while let Some(line) = lines.next_line().await? {
            tracing::trace!(?line, "New line");

            // e.g. " 99.12% depots/223354/26220720/mpmissions/dayzOffline.sakhal/mapgroupdirt.xml"
            if line.contains("% depot") {
                let mut split = line.split(' ');
                let current: f32 = split
                    .nth(1)
                    .ok_or_else(|| anyhow!("no progress in line"))?
                    .replace("%", "")
                    .parse()
                    .map_err(|_| anyhow!("progress couldn't be parsed"))?;
                let file = split.next().ok_or(anyhow!("no file in line"))?.to_owned();
                self.eventbus.send(DepotDownloadEvent::Progress { current, file })?;
            }

            // e.g. "Total downloaded: 2284736752 bytes (5790734490 bytes uncompressed) from 2 depots"
            if line.starts_with("Total downloaded:") {
                let mut split = line.split(' ');
                let total: u64 = split
                    .nth(3)
                    .ok_or_else(|| anyhow!("no bytes in line part (a)"))?
                    .parse()
                    .map_err(|_| anyhow!("bytes couldn't be parsed (a)"))?;
                let total_uncompressed: u64 = split
                    .nth(5)
                    .ok_or_else(|| anyhow!("no bytes in line part (b)"))?
                    .replace("(", "")
                    .parse()
                    .map_err(|_| anyhow!("bytes couldn't be parsed (b)"))?;
                self.eventbus.send(DepotDownloadEvent::Completed { total, total_uncompressed })?;
            }
        }

        if !child.wait().await?.success() {
            const MESSAGE: &str = "Download completed depot exited unsuccessfully";
            self.eventbus.send(DepotDownloadEvent::Failed { message: MESSAGE.to_string() })?;
            bail!("Failed depot download: {MESSAGE}");
        }
        Ok(())
    }

    // private

    #[instrument(level = "trace", skip_all, err)]
    async fn spawn_app_dl(&self, app_id: u32, manifest_only: bool) -> Result<(Child, Lines<BufReader<ChildStdout>>)> {
        create_dir_all(&self.current_dir).await?;
        let path = canonicalize(Depot::BINARY_FILEPATH).await?;
        let mut child = Command::new(path)
            .current_dir(&self.current_dir)
            .arg("-app")
            .arg(app_id.to_string())
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

// Associated functions & constants
impl Depot {
    // Binary
    pub const BINARY_FILENAME: &str = "DepotDownloader";
    pub const BINARY_FILEPATH: &str = concat!(DATA_FOLDER, "/bin/", Depot::BINARY_FILENAME);

    // DL
    const ZIP_FILENAME: &str = concat!("DepotDownloader-", OS, "-", arch_alt!(), ".zip");
    const DOWNLOAD_ZIP_BYTES: u32 = github_release_asset_bytes!(
        "https://github.com/SteamRE/DepotDownloader/releases/tag/DepotDownloader_3.4.0",
        "DepotDownloader-{OS}-{ARCH_ALT}.zip"
    );
    const DOWNLOAD_BASE_URL: &str =
        "https://github.com/SteamRE/DepotDownloader/releases/download/DepotDownloader_3.4.0";
    const DOWNLOAD_ZIP_URL: &str = concat!(Depot::DOWNLOAD_BASE_URL, "/", Depot::ZIP_FILENAME);

    pub fn new(steam_username: impl ToString, current_dir: impl ToString) -> Self {
        Self {
            steam_username: steam_username.to_string(),
            current_dir: current_dir.to_string(),
            eventbus: broadcast::channel(crate::STD_EVENT_BUFFER).0,
        }
    }

    pub fn new_login() -> DepotAuthenticator {
        DepotAuthenticator::new()
    }

    pub async fn is_logged_in(username: &str) -> Result<bool> {
        DepotAuthenticator::is_logged_in(username).await
    }

    pub async fn is_depot_downloaded() -> Result<bool> {
        std::fs::exists(Depot::BINARY_FILEPATH).map_err(Into::into)
    }

    pub fn depot_download() -> HttpDownloader {
        HttpDownloader::builder()
            .url(Depot::DOWNLOAD_ZIP_URL)
            .name(Depot::BINARY_FILENAME)
            .filename_in_archive(Depot::BINARY_FILENAME)
            .target(Depot::BINARY_FILEPATH)
            .total_download_bytes(Depot::DOWNLOAD_ZIP_BYTES as f32)
            .make_executable(true)
            .build()
    }
}
