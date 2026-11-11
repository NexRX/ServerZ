use anyhow::ensure;
use jiff::Timestamp;
use toasty::{Embed, Model, stmt::Uuid};
use topcoat::context::{Cx, memoize};
use tracing::instrument;

use crate::db::db;

#[derive(Debug, Model)]
pub struct User {
    #[key]
    id: Uuid,
    username: String,
    password_hash: String,
    role: Role,
    #[auto]
    created_at: Timestamp,
    #[auto]
    updated_at: Timestamp,
}

#[derive(Debug, Embed)]
pub enum Role {
    Owner,
    Member,
}

#[memoize(as_ref)]
#[instrument(level = "debug", skip_all, ret)]
pub async fn ensure_owner_created(cx: &Cx) -> anyhow::Result<()> {
    let is_created = User::filter(User::fields().role().eq(Role::Owner))
        .count()
        .exec(&mut db(cx))
        .await
        .map(|count| count > 0)?;
    ensure!(is_created, "No user is created");

    Ok(())
}
