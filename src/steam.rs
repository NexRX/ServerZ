mod depot;
mod depot_signin;

use constcat::concat;
pub use depot::*;
pub use depot_signin::*;

use crate::{DATA_FOLDER, crypto::Encrypted, db::Singleton, guard::Guard};
use anyhow::ensure;
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
