use crate::{DATA_FOLDER, download::Downloader};
use constcat::concat;
use server_z_macros::{arch_alt, github_release_asset_bytes};
use std::env::consts::OS;

pub struct Depot {}

/// Associated functions & constants
impl Depot {
    const BINARY_FILENAME: &str = "DepotDownloader";
    const BINARY_FILEPATH: &str = concat!(DATA_FOLDER, "/bin/", Depot::BINARY_FILENAME);

    const ZIP_FILENAME: &str = concat!("DepotDownloader-", OS, "-", arch_alt!(), ".zip");
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

    pub fn downloader() -> Downloader {
        Downloader::builder()
            .url(Depot::DOWNLOAD_ZIP_URL)
            .name(Depot::BINARY_FILENAME)
            .filename_in_archive(Depot::BINARY_FILENAME)
            .target(Depot::BINARY_FILEPATH)
            .total_download_bytes(Depot::DOWNLOAD_ZIP_BYTES)
            .build()
    }
}
