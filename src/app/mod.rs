pub mod showcase;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    font::{Font, fontsource::fontsource_font},
    router::{RouterBuilder, RouterBuilderDiscoverExt, Slot, layout, module_router, page},
    runtime::RouterBuilderRuntimeExt,
    tailwind,
    view::{View, view},
};

/// The Geist font family, loaded from Fontsource.
const GEIST: Font = fontsource_font!(GEIST, weight: [400, 700], style: Normal);

/// Builds the application's router.
pub fn router() -> RouterBuilder {
    // The pages and layouts of this module and its submodules, with paths derived
    // from the module tree.
    module_router!()
        // The runtime's procedures and shards, which have no place in the module tree.
        .discover()
        .assets(AssetBundle::load().unwrap())
        // Lets the browser runtime rerun pages; register it after other layers.
        .runtime()
}

#[layout]
async fn root(slot: Slot<'_>) -> Result<impl View> {
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
            <body>(slot)</body>
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
