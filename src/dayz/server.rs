use crate::{
    dayz::{launch_params::LaunchParams, mods::Mod},
    db::Singleton,
    guard::Guard,
};
use jiff::Timestamp;
use toasty::{Executor, Model};
use topcoat::context::Cx;

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

impl Guard for Server {
    const GUARD_NAME: &str = "Server";
    const REDIRECT_AVAILABLE: &str = "/setup/owner";
    const REDIRECT_UNAVAILABLE: &str = "/setup/server";

    async fn fetch_available(cx: &Cx) -> anyhow::Result<Self> {
        Server::fetch(cx).await.map_err(Into::into)
    }
}
