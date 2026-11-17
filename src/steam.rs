use crate::{
    crypto::Encrypted,
    db::{Singleton, db},
};
use jiff::Timestamp;
use toasty::{Executor, Model, stmt::Uuid};
use topcoat::context::{Cx, memoize};
use tracing::instrument;

#[derive(Debug, Clone, Model)]
pub struct Steam {
    #[key]
    #[default(Steam::SINGLETON_ID)]
    pub id: u8,
    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,

    pub auth_user: String,
    pub auth_password: Encrypted,
}

impl Singleton for Steam {
    async fn get(db: &mut dyn Executor) -> toasty::Result<Self> {
        Self::get_by_id(db, Self::SINGLETON_ID).await
    }
}
