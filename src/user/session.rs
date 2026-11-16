use std::time::UNIX_EPOCH;

use crate::{anyhow, db::db, user::User};
use jiff::Timestamp;
use toasty::{Model, stmt::Uuid};
use topcoat::{context::Cx, session::TokenHash};

#[derive(Debug, Clone, Model)]
pub struct Session {
    #[key]
    #[auto]
    pub id: Uuid,
    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,

    #[index]
    pub token_hash: Vec<u8>,
    pub expires_at: Timestamp,

    #[index]
    user_id: Uuid,
    #[belongs_to(key = user_id, references = id)]
    user: toasty::Deferred<User>,
}

impl Session {
    pub async fn persist(
        cx: &Cx,
        user: &User,
        session: topcoat::session::Session,
    ) -> topcoat::Result<()> {
        let expires_at = session
            .expires_at
            .duration_since(UNIX_EPOCH)
            .map(|v| Timestamp::from_second(v.as_secs() as i64))
            .map_err(|err| {
                anyhow!("Failed to convert session expiry to unix timestamp: {err:?}")
            })??;

        toasty::create!(Session {
            token_hash: session.token_hash.to_vec(),
            expires_at,
            user
        })
        .exec(&mut db(cx))
        .await?;
        Ok(())
    }

    pub async fn fetch_active(cx: &Cx, token_hash: &TokenHash) -> topcoat::Result<Option<Self>> {
        let Some(session) = Session::filter(
            Session::fields()
                .token_hash()
                .eq(token_hash.to_vec())
                .and(Session::fields().expires_at().ge(Timestamp::now())),
        )
        .first()
        .exec(&mut db(cx))
        .await?
        else {
            return Ok(None);
        };

        Ok(Some(session))
    }
}
