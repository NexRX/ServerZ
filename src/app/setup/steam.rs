use crate::components::prelude::*;
use crate::crypto::Secret;
use crate::download::ReceiveLatest as _;
use crate::guard::Guard;
use crate::steam::{Depot, Steam};
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use server_z_macros::FormValidation;
use topcoat::router::content::Form;
use topcoat::router::error::SeeOther;
use topcoat::router::{Route, route};
use topcoat::runtime::{Event, connected, signal};
use topcoat::view::{attributes, component, emit, live};
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

pub fn is_true(v: &bool, _: &&&()) -> garde::Result {
    if !*v {
        return Err(garde::Error::new("depot must be downloaded"));
    }
    Ok(())
}

#[instrument(level = "info", skip(cx), ret, err)]
#[route(POST "/api/setup/steam")]
async fn create_steam(cx: &Cx, Form(input): Form<SteamDepot>) -> Result<SeeOther> {
    input.validate()?;
    todo!()
}

#[page]
pub async fn setup_steam(cx: &Cx) -> Result<impl View> {
    Steam::ensure_unavailable(cx).await?;

    let btn_state = match Depot::is_downloaded().await.unwrap_or_default() {
        true => DownloadProgressBtn::Done,
        false => DownloadProgressBtn::Enabled,
    };

    Ok(view! {
        <form
            method="POST"
            action=(create_steam.path().to_string())
            class="flex flex-col flex-1 items-center h-full gap-4"
        >
            download_progress(
                downloader: Depot::downloader(),
                start: signal(cx, || false),
                fieldset: Some(
                    (
                        "Required Download: Depot",
                        "This tool is required for downloading items from steam",
                    ),
                ),
                start_button: btn_state,
                form_marker: Some("depot_downloaded")
            )
            depot_signin_qr()
            button(
                attrs: attributes!(type="submit" class="mt-auto ml-auto self-end"),
                "Next"
                <div class="iconify solar--arrow-right-bold-duotone" />
            )
        </form>
    })
}

#[component]
pub async fn depot_signin_qr(cx: &Cx) -> Result<impl View> {
    let start = signal(cx, || false);
    Ok(view! {
        (live! {
            let signin = Depot::new_signin();
            let mut sub = signin.subscribe();

            if start.get() {
                signin.start_bg();
                emit! { <p>"Loading..."</p> }?;
            }

            if !connected(cx) {
                return emit! {
                    button(
                        ty: ButtonType::Button,
                        attrs: attributes!(
                            @click=$(async |_e: Event| {
                                start.set(true);
                            })
                        ),
                        "Get QR"
                    )
                };
            }

            while let Some(event) = sub.recv_latest().await {
                match event {
                    crate::steam::DepotSigninEvent::QR(qr) => emit! {
                        <p
                            class="font-mono leading-none tracking-normal whitespace-pre text-xs"
                        >
                            (qr)
                        </p>
                    }?,
                    crate::steam::DepotSigninEvent::Failed => return emit! { <p class="Failure">"Failed to scan QR in time"</p> },
                };
            };

            emit! { <p class="Failure">"Failed to scan QR in time"</p> }
        })
    })
}
