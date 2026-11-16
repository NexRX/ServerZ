use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use server_z_macros::FormValidation;
use topcoat::{
    Result,
    context::{Cx, memoize},
    router::{
        content::Form,
        error::{SeeOther, see_other},
        href, route,
    },
    session,
};
use tracing::instrument;

use crate::{
    anyhow,
    crypto::Secret,
    db::db,
    user::{Session, User},
};

#[derive(Debug, JsonSchema, Validate, FormValidation, Serialize, Deserialize, Hash)]
pub struct Login {
    #[garde(ascii, length(min = 3, max = 30))]
    pub username: String,
    #[garde(dive)]
    pub password: Secret,
}

#[route(POST "/api/auth/login")]
async fn login(cx: &Cx, Form(input): Form<Login>) -> Result<SeeOther> {
    let user = verify_login(cx, &input)
        .await?
        .ok_or(anyhow!("Login not valid"))?;

    Session::persist(cx, &user, session::start(cx).await?).await?;

    Ok(see_other(href!(crate::app::home).resolve(cx)))
}

#[route(POST "/api/auth/logout")]
async fn logout(cx: &Cx) -> Result<SeeOther> {
    if let Some(hash) = session::stop(cx).await? {
        Session::delete_by_token_hash(&mut db(cx), hash.to_vec()).await?;
    }
    Ok(see_other(href!(crate::app::home).resolve(cx)))
}

async fn verify_login(cx: &Cx, input: &Login) -> Result<Option<User>> {
    let Some(user) = User::filter(User::fields().username().eq(&input.username))
        .first()
        .exec(&mut db(cx))
        .await?
    else {
        return Ok(None);
    };

    match input.password.verify_hash(&user.password_hash) {
        true => Ok(Some(user)),
        false => Ok(None),
    }
}

/// Returns the current user
#[instrument(level = "debug", skip_all, ret, err)]
#[memoize(as_ref)]
pub async fn me(cx: &Cx) -> Result<Option<User>> {
    let Some(hash) = session::token_hash(cx).await? else {
        return Ok(None);
    };
    let Some(session) = Session::fetch_active(cx, &hash).await? else {
        return Ok(None);
    };

    session
        .user()
        .exec(&mut db(cx))
        .await
        .map(Option::Some)
        .map_err(Into::into)
}

/// Returns the current user
pub async fn me_cloned(cx: &Cx) -> Result<Option<User>> {
    Ok(me(cx).await.map(|v| match v {
        Some(user) => Some(user.clone()),
        None => None,
    })?)
}
