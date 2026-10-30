mod app;
mod components;
mod db;
mod error;
mod features;
mod server;

use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    db::handle_cli_migration().await;

    info!("Starting");
    let db = db::try_connect().await?;
    db::migrate(&db).await?;

    topcoat::start(app::router().app_context(db).build())
        .await
        .unwrap();
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
