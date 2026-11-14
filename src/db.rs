use toasty::Db;
use toasty::Executor;
use toasty::Result;
use toasty::migration::MigrationReport;
use topcoat::context::Cx;
use topcoat::context::context;
use tracing::error;
use tracing::info;
use tracing::instrument;

const DATABASE_URL: &str = "sqlite:data.db";

#[instrument(level = "debug", err)]
pub async fn try_connect() -> Result<Db> {
    Db::builder()
        .models(toasty::models!(crate::*))
        .connect(DATABASE_URL)
        .await
}

pub async fn connect_or_exit() -> Db {
    match try_connect().await {
        Ok(pool) => pool,
        Err(err) => {
            error!(?err, "Failed to connect to database so exiting");
            std::process::exit(crate::error::CODE_DB_CONNECTION);
        }
    }
}

/// Returns the application database handle.
pub fn db(cx: &Cx) -> Db {
    context::<Db>(cx).clone()
}

/// `cargo run -- toasty migration <command>` manages the database schema.
pub async fn handle_cli_migration() {
    let mut args = std::env::args().skip(1).peekable();

    let is_handling = args.peek().is_some_and(|arg| arg == "toasty");
    if is_handling {
        info!("CLI: Running migrations");
        match toasty_cli::ToastyCli::new(connect_or_exit().await)
            .parse_from(args)
            .await
        {
            Ok(_) => {
                info!("Migrations ran successfully, exiting...");
                std::process::exit(0);
            }
            Err(err) => {
                error!("{err:#}");
                std::process::exit(crate::error::CODE_DB_MIGRATION);
            }
        }
    }
}

#[instrument(level = "info", skip_all, ret, err)]
pub async fn migrate(db: &Db) -> toasty::Result<MigrationReport> {
    static MIGRATIONS: toasty::migration::MigrationSet = toasty::embed_migrations!();
    MIGRATIONS.apply(db).await
}

pub trait Singleton: Sized {
    const SINGLETON_ID: u8 = 0;

    async fn get(db: &mut dyn Executor) -> Result<Self>;
    #[allow(dead_code)]
    async fn fetch(cx: &Cx) -> Result<Self> {
        Self::get(&mut db(cx)).await
    }
}
