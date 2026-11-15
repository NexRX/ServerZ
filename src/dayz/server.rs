use crate::{
    dayz::{launch_params::LaunchParams, mods::Mod},
    db::{Singleton, db},
};
use anyhow::ensure;
use jiff::Timestamp;
use toasty::{Executor, Model};
use topcoat::context::{Cx, memoize};
use tracing::instrument;

#[derive(Debug, Model)]
pub struct Server {
    #[key]
    #[default(Server::SINGLETON_ID)]
    id: u8,
    #[auto]
    created_at: Timestamp,
    #[auto]
    updated_at: Timestamp,

    #[has_one]
    launch_params: LaunchParams,
    #[has_many]
    mods: Vec<Mod>,
}

impl Singleton for Server {
    async fn get(db: &mut dyn Executor) -> toasty::Result<Self> {
        Server::get_by_id(db, Server::SINGLETON_ID).await
    }
}

#[memoize]
#[instrument(level = "trace", skip_all, ret)]
pub async fn is_server_setup(cx: &Cx) -> bool {
    Server::get(&mut db(cx)).await.is_ok()
}

#[memoize(as_ref)]
#[instrument(level = "debug", skip_all, ret)]
pub async fn ensure_server_created(cx: &Cx) -> anyhow::Result<()> {
    ensure!(*is_server_setup(cx).await, "No server is created");
    Ok(())
}

#[memoize(as_ref)]
#[instrument(level = "debug", skip_all, ret)]
pub async fn ensure_server_not_created(cx: &Cx) -> anyhow::Result<()> {
    ensure!(!*is_server_setup(cx).await, "Server is already created");
    Ok(())
}
