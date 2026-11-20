mod session;

use crate::{app::api::auth::me_cloned, db::db, guard::Guard};
use anyhow::anyhow;
use derive_more::{Deref, DerefMut};
use jiff::Timestamp;
pub use session::*;
use toasty::{Embed, Model, stmt::Uuid};
use topcoat::context::Cx;

#[derive(Debug, Clone, Model)]
pub struct User {
    #[key]
    #[auto]
    pub id: Uuid,
    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,

    pub username: String,
    pub password_hash: String,
    pub role: Role,

    #[has_many]
    sessions: Vec<Session>,
}

#[derive(Debug, Copy, Clone, Embed, Default)]
pub enum Role {
    Owner,
    Member,
    #[default]
    Guest,
}

impl Guard for User {
    const GUARD_NAME: &str = "User";
    const REDIRECT_UNAVAILABLE: &str = "/login";

    async fn fetch_available(cx: &Cx) -> anyhow::Result<Self> {
        me_cloned(cx).await.map_err(|err| anyhow!("{err:?}"))?.ok_or_else(|| anyhow!("Not logged in"))
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Owner(pub User);

impl Guard for Owner {
    const GUARD_NAME: &str = "Owner";
    const REDIRECT_UNAVAILABLE: &str = "/setup/owner";

    async fn fetch_available(cx: &Cx) -> anyhow::Result<Self> {
        User::filter(User::fields().role().eq(Role::Owner))
            .first()
            .exec(&mut db(cx))
            .await
            .map_err(|err| anyhow!("Failed to fetch owner: {err:?}"))?
            .map(Owner)
            .ok_or_else(|| anyhow!("No owner account found"))
    }
}
