use super::SetupStep;

use crate::components::button::button;
use crate::dayz::ensure_server_created;
use crate::user::ensure_owner_created;
use topcoat::router::error::RouterErrorExt as _;
use topcoat::router::href;
use topcoat::view::attributes;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

#[page]
pub async fn setup_done(cx: &Cx) -> Result<impl View> {
    ensure_owner_created(cx)
        .await
        .ok_or_redirect(SetupStep::Owner.to_uri())?;
    ensure_server_created(cx)
        .await
        .ok_or_redirect(SetupStep::Server.to_uri())?;

    Ok(view! {
        <div class="flex flex-1 items-center justify-center">
            <div
                class="setup-success-circle relative flex h-32 w-32 items-center justify-center rounded-full bg-success"
                role="img"
                aria-label="Setup complete"
            >
                <div
                    class="setup-success-ring pointer-events-none absolute inset-0 rounded-full bg-success"
                ></div>
                <div
                    class="setup-success-check absolute inset-0 z-10 flex items-center justify-center"
                >
                    <div class="iconify solar--check-bold text-6xl text-white"></div>
                </div>
            </div>
        </div>
        <link rel="prefetch" href=(href!(crate::app::home))>
        <a href=(href!(crate::app::home)) class="link mt-auto ml-auto self-end">
            button(
                attrs: attributes!(type="button"),
                "Continue to homepage"
                <div class="iconify solar--arrow-right-bold-duotone" />
            )
        </a>
    })
}
