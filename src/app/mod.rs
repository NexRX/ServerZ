pub mod api;
pub mod login;
pub mod setup;
pub mod showcase;

use crate::app::setup::ensure_setup;
use crate::components::prelude::*;
use crate::dayz::SERVER_PATH;
use crate::db::Singleton;
use crate::features::app_menu::app_menu;
use crate::guard::Guard;
use crate::steam::{DAYZ_SERVER_APPID, Steam};
use crate::user::User;
use toasty::Db;
use topcoat::Error;
use topcoat::cookie::CookieLayer;
use topcoat::runtime::{Event, procedure};
use topcoat::view::attributes;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    font::{Font, fontsource::fontsource_font},
    router::{Router, RouterBuilderDiscoverExt, Slot, error::NotFoundError, layout, module_router, page, request::uri},
    runtime::RuntimeLayer,
    tailwind,
    view::{View, component, error_boundary, view},
};
use tracing::{instrument, trace};

topcoat::router::not_found!();

pub fn router(db: Db) -> Router {
    module_router!()
        .discover()
        .with(db)
        .assets(AssetBundle::load().unwrap())
        .layer(RuntimeLayer::new())
        .layer(CookieLayer::new())
        .build()
}

#[layout]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    const GEIST: Font = fontsource_font!(GEIST, weight: [400, 700], style: Normal);
    let path = uri(cx).path();
    let is_hide_menu = path.starts_with("/setup");

    let path = uri(cx).path();

    let is_require_setup = !(path.starts_with("/setup") || path.starts_with("/showcase"));
    if is_require_setup {
        trace!(?path, "Requiring setup");
        ensure_setup(cx).await?;
    }

    let is_require_login = !(!is_require_setup || path.starts_with("/login"));
    if is_require_login {
        trace!(?is_require_setup, ?path, "Requiring login");
        User::ensure_available(cx).await?;
    }

    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"ServerZ"</title>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                <script type="module" src="https://unpkg.com/cally"></script>
                topcoat::font::link(font: GEIST)
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body>
                error_boundary(
                    fallback: |error| {
                        if error.downcast_ref::<NotFoundError>().is_some() {
                            Ok(view! { app_menu(not_found_fallback()) })
                        } else {
                            Err(error)
                        }
                    },
                    app_menu(hide: is_hide_menu, (slot))
                )
            </body>
        </html>
    })
}

#[instrument(level = "debug", skip_all, ret, err)]
#[procedure]
pub async fn app_manifest(cx: &Cx) -> Result<()> {
    let depot = Steam::fetch(cx).await?.depot(SERVER_PATH);
    depot.download_app(DAYZ_SERVER_APPID).await.map_err(Error::from_anyhow)?;

    Ok(())
}

#[page]
pub async fn home(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <main class="mx-auto max-w-2xl px-4 py-16">
            "hi"
            button(
                attrs: attributes!(
                    @click=$(async |_e: Event| {
                        app_manifest().await;
                    })
                ),
                "Button"
            )
        </main>
    })
}

#[component]
async fn not_found_fallback(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <main class="mx-auto max-w-2xl px-4 py-16">
            "Not Found! "
            <code>(uri(cx).path())</code>
            " does not exist."
        </main>
    })
}
