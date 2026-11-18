use super::SetupStep;
use crate::app::api::auth::{Login, LoginValidation};
use crate::components::prelude::*;
use crate::db::db;
use crate::steam::ensure_steam_is_not_setup;
use crate::user::{Role, Session, User, ensure_owner_not_created};
use crate::{ensure, user::ensure_owner_created};
use garde::Validate;
use topcoat::router::error::{RouterErrorExt as _, SeeOther, see_other};
use topcoat::runtime::{Event, procedure};
use topcoat::session;
use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, page, route},
    view::{View, attributes, view},
};
use tracing::instrument;

#[procedure]
#[instrument(level = "info", skip_all)]
async fn start_download(cx: &Cx) -> Result<usize> {
    ensure_steam_is_not_setup(cx).await.ok_or_redirect(SetupStep::Steam.next().to_uri())?;

    Ok(1 * 2)
}

#[page]
pub async fn setup_steam(cx: &Cx) -> Result<impl View> {
    ensure_steam_is_not_setup(cx).await.ok_or_redirect(SetupStep::Steam.next().to_uri())?;

    Ok(view! {
        "Start Steam Download?"
        button(
            attrs: attributes! {
                @click=$(async |_e: Event| {
                    start_download().await;
                })
            },
            "Start"
        )
    })
}
