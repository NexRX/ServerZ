use crate::components::button::button;
use crate::dayz::Server;
use crate::guard::Guard as _;
use crate::steam::Steam;
use crate::user::Owner;
use topcoat::router::error::RedirectError;
use topcoat::router::href;
use topcoat::view::attributes;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

pub async fn ensure_setup(cx: &Cx) -> Result<(), RedirectError> {
    Owner::ensure_available(cx).await?;
    Server::ensure_available(cx).await?;
    Steam::ensure_available(cx).await?;
    Ok(())
}

#[page]
pub async fn setup_done(cx: &Cx) -> Result<impl View> {
    ensure_setup(cx).await?;

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
