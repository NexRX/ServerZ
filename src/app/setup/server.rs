use crate::app::setup::SetupStep;
use crate::components::prelude::*;
use crate::crypto::Secret;
use crate::dayz::server::{ensure_server_created, ensure_server_not_created};
use crate::ensure;
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use server_z_macros::FormValidation;
use topcoat::router::error::{RouterErrorExt as _, SeeOther, see_other};
use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, page, route},
    view::{View, attributes, view},
};
use tracing::instrument;

#[derive(Debug, JsonSchema, Validate, FormValidation, Serialize, Deserialize)]
struct ServerCreation {
    #[garde(dive)]
    placeholder: Secret,
}

#[instrument(level = "info", skip(cx), ret, err)]
#[route(POST "/api/setup/create-server")]
async fn create_owner(cx: &Cx, Form(input): Form<ServerCreation>) -> Result<SeeOther> {
    ensure!(
        ensure_server_created(cx).await.is_err(),
        "Server has already been created"
    )?;
    input.validate()?;

    Ok(see_other(SetupStep::Done.to_uri()))
}

#[page]
pub async fn setup_owner(cx: &Cx) -> Result<impl View> {
    ensure_server_not_created(cx)
        .await
        .ok_or_redirect(SetupStep::Done.to_uri())?;

    Ok(view! {
        <form
            method="POST"
            action="/api/setup/create-server"
            class="flex flex-col flex-1 items-center h-full"
        >
            button(
                attrs: attributes!(type="submit" class="mt-auto ml-auto self-end"),
                "Next"
                <div class="iconify solar--arrow-right-bold-duotone" />
            )
        </form>
    })
}
