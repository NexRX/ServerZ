#![allow(unused_attributes)]

mod app;
mod components;
mod crypto;
mod dayz;
mod db;
mod error;
mod features;
mod steam;
mod user;

use tracing::info;

#[cfg(debug_assertions)]
pub const DATA_FOLDER: &str = "./data";
#[cfg(not(debug_assertions))]
pub const DATA_FOLDER: &str = "/var/lib/serverz";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    std::fs::create_dir_all(DATA_FOLDER)?;
    db::handle_cli_migration().await;

    info!("Starting ServerZ");
    let db = db::try_connect().await?;
    db::migrate(&db).await?;

    topcoat::start(app::router(db)).await.unwrap();
    Ok(())
}

fn init_logging() {
    #[cfg(debug_assertions)]
    const DEFAULT_LOGGING: &str = "serverz=debug";
    #[cfg(not(debug_assertions))]
    const DEFAULT_LOGGING: &str = "serverz=info";

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(DEFAULT_LOGGING)),
        )
        .with_target(false)
        .init();
}
