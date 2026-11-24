mod depot;
mod depot_auth;

pub use depot::*;
pub use depot_auth::*;
use tracing::instrument;

use crate::{DATA_FOLDER, db::Singleton, guard::Guard};
use anyhow::Result;
use anyhow::ensure;
use constcat::concat;
use jiff::Timestamp;
use toasty::{Executor, Model};
use topcoat::context::Cx;

const DAYZ_SERVER_APPID: u32 = 223350;
const STEAM_DATA_FOLDER: &str = concat!(DATA_FOLDER, "/steamdepot");

#[derive(Debug, Clone, Model)]
pub struct Steam {
    #[key]
    #[default(Steam::SINGLETON_ID)]
    pub id: u8,
    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,

    pub steam_username: String,
    pub is_authenticated: bool,
}

impl Steam {
    #[instrument(level = "trace", skip_all, ret, err)]
    pub async fn is_depot_logged_in(&self) -> Result<bool> {
        Depot::is_logged_in(&self.steam_username).await
    }
}

impl Singleton for Steam {
    async fn get(db: &mut dyn Executor) -> toasty::Result<Self> {
        Self::get_by_id(db, Self::SINGLETON_ID).await
    }
}

impl Guard for Steam {
    const REDIRECT_AVAILABLE: &str = "/setup/done";
    const REDIRECT_UNAVAILABLE: &str = "/setup/steam";

    async fn fetch_available(cx: &Cx) -> anyhow::Result<Self> {
        ensure!(Depot::is_downloaded().await?, "Depot is not downloaded");
        Self::fetch(cx).await.map_err(Into::into)
    }
}
