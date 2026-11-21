use crate::components::prelude::*;
use crate::crypto::Secret;
use crate::guard::Guard;
use crate::steam::{Depot, Steam};
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use server_z_macros::FormValidation;
use topcoat::router::content::Form;
use topcoat::router::error::SeeOther;
use topcoat::router::route;
use topcoat::runtime::signal;
use topcoat::view::attributes;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};
use tracing::instrument;

#[derive(Debug, JsonSchema, Validate, FormValidation, Serialize, Deserialize, Hash)]
pub struct SteamDepot {
    #[garde(ascii, length(min = 3, max = 30))]
    pub steam_username: String,
    #[garde(dive)]
    pub steam_password: Secret,
}

#[instrument(level = "info", skip(cx), ret, err)]
#[route(POST "/api/setup/steam")]
async fn create_steam(cx: &Cx, Form(input): Form<SteamDepot>) -> Result<SeeOther> {
    todo!()
}

#[page]
pub async fn setup_steam(cx: &Cx) -> Result<impl View> {
    Steam::ensure_unavailable(cx).await?;

    let btn_state = if Depot::is_downloaded().await.unwrap_or_default() {
        DownloadProgressBtn::Done
    } else {
        DownloadProgressBtn::Enabled
    };

    Ok(view! {
        download_progress(
            downloader: Depot::downloader(),
            start: signal(cx, || false),
            fieldset: Some(
                (
                    "Required Download: Depot",
                    "This tool is required for downloading items from steam",
                ),
            ),
            start_button: btn_state
        )
        <form
            method="POST"
            action="/api/setup/create-owner"
            class="flex flex-col flex-1 items-center h-full"
        >
            input(
                label: Some("Steam Username"),
                label_for: Some(SteamDepotValidation::SteamUsername.name()),
                inner_attrs: SteamDepotValidation::SteamUsername.attrs(cx),
                attrs: attributes!(class="w-72")
            )
            input(
                ty: InputType::Password,
                label: Some("Steam Password"),
                label_for: Some(SteamDepotValidation::SteamPassword.name()),
                inner_attrs: SteamDepotValidation::SteamPassword.attrs(cx),
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
