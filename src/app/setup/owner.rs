use super::SetupStep;
use crate::app::api::auth::{Login, LoginValidation};
use crate::components::prelude::*;
use crate::db::db;
use crate::user::{Role, Session, User, ensure_owner_not_created};
use crate::{ensure, user::ensure_owner_created};
use garde::Validate;
use topcoat::router::error::{RouterErrorExt as _, SeeOther, see_other};
use topcoat::session;
use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, page, route},
    view::{View, attributes, view},
};
use tracing::instrument;

#[instrument(level = "info", skip(cx), ret, err)]
#[route(POST "/api/setup/create-owner")]
async fn create_owner(cx: &Cx, Form(input): Form<Login>) -> Result<SeeOther> {
    ensure!(ensure_owner_created(cx).await.is_err(), "Owner has already been created")?;
    input.validate()?;

    let user =
        toasty::create!(User { username: input.username, password_hash: input.password.hash()?, role: Role::Owner })
            .exec(&mut db(cx))
            .await?;

    // save session so they dont have to login post-setup
    Session::persist(cx, &user, session::start(cx).await?).await?;

    Ok(see_other(SetupStep::Server.to_uri()))
}

#[page]
pub async fn setup_owner(cx: &Cx) -> Result<impl View> {
    ensure_owner_not_created(cx).await.ok_or_redirect(SetupStep::Server.to_uri())?;

    Ok(view! {
        <form
            method="POST"
            action="/api/setup/create-owner"
            class="flex flex-col flex-1 items-center h-full"
        >
            input(
                label: Some("Username".into()),
                label_for: Some(LoginValidation::Username.name().into()),
                inner_attrs: LoginValidation::Username.attrs(cx),
                attrs: attributes!(class="w-72")
            )
            input(
                ty: InputType::Password,
                label: Some("Password".into()),
                label_for: Some(LoginValidation::Password.name().into()),
                inner_attrs: LoginValidation::Password.attrs(cx),
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
