use jiff::Timestamp;
use miette::ensure;
use toasty::{Executor, Model};
use topcoat::context::{Cx, memoize};
use tracing::instrument;

use crate::db::{Singleton, db};

#[derive(Debug, Model)]
pub struct Server {
    #[key]
    id: u8,
    profile_path: String,
    server_path: String,
    port: u16,
    cpu_count: u8,
    battleye: bool,
    #[auto]
    created_at: Timestamp,
    #[auto]
    updated_at: Timestamp,
}

impl Singleton for Server {
    async fn get(db: &mut dyn Executor) -> toasty::Result<Self> {
        Server::get_by_id(db, Server::SINGLETON_ID).await
    }
}

#[memoize]
#[instrument(level = "debug", skip_all, ret)]
pub async fn is_server_setup(cx: &Cx) -> bool {
    Server::get(&mut db(cx)).await.is_ok()
}

#[memoize(as_ref)]
#[instrument(level = "debug", skip_all, ret)]
pub async fn ensure_server_created(cx: &Cx) -> miette::Result<()> {
    ensure!(*is_server_setup(cx).await, "No server is created");
    Ok(())
}
