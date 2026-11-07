pub mod setup;
pub mod showcase;

use crate::features::{app_menu::app_menu, guards::require_server};
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    font::{Font, fontsource::fontsource_font},
    router::{
        RouterBuilder, RouterBuilderDiscoverExt, Slot, error::NotFoundError, layout, module_router,
        page, request::uri,
    },
    runtime::RouterBuilderRuntimeExt,
    tailwind,
    view::{View, component, error_boundary, view},
};

topcoat::router::not_found!();

pub fn router() -> RouterBuilder {
    module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .runtime()
}

#[layout]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    const GEIST: Font = fontsource_font!(GEIST, weight: [400, 700], style: Normal);

    let path = uri(cx).path();
    if !path.starts_with("/setup") && path.starts_with("/showcase") {
        require_server(cx).await?;
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
                    app_menu((slot))
                )
            </body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <main class="mx-auto max-w-2xl px-4 py-16">"hi"</main> })
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
