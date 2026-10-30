pub mod setup;
pub mod showcase;

use crate::features::{app_menu::app_menu, guards::require_server};
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt, asset},
    context::Cx,
    font::{Font, fontsource::fontsource_font},
    router::{
        RouterBuilder, RouterBuilderDiscoverExt, Slot, layout, module_router, page, request::uri,
    },
    runtime::RouterBuilderRuntimeExt,
    tailwind,
    view::{View, view},
};

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
                app_menu((slot))
           </body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <main class="mx-auto max-w-2xl px-4 py-16">
           "hi"
        </main>
    })
}
