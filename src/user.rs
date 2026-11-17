mod session;

pub use session::*;

use anyhow::ensure;
use jiff::Timestamp;
use toasty::{Embed, Model, stmt::Uuid};
use topcoat::context::{Cx, memoize};
use tracing::instrument;

use crate::db::db;

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

#[memoize(as_ref)]
#[instrument(level = "debug", skip_all, ret)]
pub async fn ensure_owner_created(cx: &Cx) -> anyhow::Result<()> {
    let is_created =
        User::filter(User::fields().role().eq(Role::Owner)).count().exec(&mut db(cx)).await.map(|count| count > 0)?;
    ensure!(is_created, "No user is created");

    Ok(())
}

#[memoize(as_ref)]
#[instrument(level = "debug", skip_all, ret)]
pub async fn ensure_owner_not_created(cx: &Cx) -> anyhow::Result<()> {
    let is_not_created =
        User::filter(User::fields().role().eq(Role::Owner)).count().exec(&mut db(cx)).await.map(|count| count == 0)?;
    ensure!(is_not_created, "User is already created");

    Ok(())
}
