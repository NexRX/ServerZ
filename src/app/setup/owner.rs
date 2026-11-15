use super::SetupStep;
use crate::components::prelude::*;
use crate::crypto::Secret;
use crate::db::db;
use crate::user::{Role, User, ensure_owner_not_created};
use crate::{ensure, user::ensure_owner_created};
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
struct CreateOwner {
    #[garde(ascii, length(min = 3, max = 30))]
    username: String,
    #[garde(dive)]
    password: Secret,
}

#[instrument(level = "info", skip(cx), ret, err)]
#[route(POST "/api/setup/create-owner")]
async fn create_owner(cx: &Cx, Form(input): Form<CreateOwner>) -> Result<SeeOther> {
    ensure!(
        ensure_owner_created(cx).await.is_err(),
        "Owner has already been created"
    )?;
    input.validate()?;

    toasty::create!(User {
        username: input.username,
        password_hash: input.password.0,
        role: Role::Owner,
    })
    .exec(&mut db(cx))
    .await?;

    Ok(see_other(SetupStep::Server.to_uri()))
}

#[page]
pub async fn setup_owner(cx: &Cx) -> Result<impl View> {
    ensure_owner_not_created(cx)
        .await
        .ok_or_redirect(SetupStep::Server.to_uri())?;

    Ok(view! {
        <form
            method="POST"
            action="/api/setup/create-owner"
            class="flex flex-col flex-1 items-center h-full"
        >
            input(
                label: Some("Username".into()),
                label_for: Some(CreateOwnerValidation::Username.name().into()),
                inner_attrs: CreateOwnerValidation::Username.attrs(cx),
                attrs: attributes!(class="w-72")
            )
            input(
                ty: InputType::Password,
                label: Some("Password".into()),
                label_for: Some(CreateOwnerValidation::Password.name().into()),
                inner_attrs: CreateOwnerValidation::Password.attrs(cx),
                attrs: attributes!(class="w-72")
            )
            button(
                attrs: attributes!(type="submit" class="mt-auto ml-auto self-end"),
                "Next"
                <div class="iconify solar--arrow-right-bold-duotone" />
            )
        </form>
    })
}
