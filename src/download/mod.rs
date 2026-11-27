mod depot_app;
mod http;

pub use depot_app::*;
pub use http::*;

use derive_more::{Display, IsVariant};
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;

pub trait Downloader {
    fn subscribe(&self) -> Receiver<DownloadEvent>;
    fn start_bg(&self);
    fn has_extract_step(&self) -> bool;
    fn name(&self) -> String;
}

#[derive(Debug, Clone, Display, IsVariant)]
pub enum DownloadEvent {
    #[display("Failed download '{name}': {message}")]
    Failed { name: Arc<str>, message: String },
    #[display("Downloading '{name}', ({current} out of {total} bytes)")]
    Downloading { name: Arc<str>, current_file: Arc<str>, has_extract_step: bool, current: f32, total: f32 },
    #[display("Extracting '{name}', ({current} out of {total} bytes)")]
    Extracting { name: Arc<str>, current: f32, total: f32 },
    #[display("Completed '{name}' ({total} bytes)")]
    Completed { name: Arc<str>, total: f32 },
}
