use crate::components::prelude::*;
use crate::download::ReceiveLatest as _;
use crate::guard::Guard;
use crate::steam::{Depot, Steam};
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use server_z_macros::FormValidation;
use topcoat::Error;
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
    #[garde(custom(is_true))]
    pub depot_downloaded: bool,
}

pub fn is_true(v: &bool, _: &&&()) -> garde::Result {
    match !*v {
        true => Ok(()),
        false => Err(garde::Error::new("depot must be downloaded")),
    }
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
                fieldset: (
                    "Required Download: Depot",
                    "This tool is required for downloading items from steam",
                ),
                start_button: btn_state,
                form_marker: "depot_downloaded"
            )
            fieldset(legend: "Steam Authentication", bordered: true, depot_signin_qr())
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
    let button_start = start.clone();
    let started = start.get();

    let button = view! {
        button(
            ty: ButtonType::Button,
            attrs: attributes!(
                @click=$(async |_e: Event| {
                    button_start.set(true);
                })
            ),
            "Get QR"
        )
    };

    Ok(view! {
        (live! {
            if !started {
                return emit! {
                    steam_username()
                    (button)
                };
            }

            if !Depot::is_downloaded().await.map_err(Error::from_anyhow)? {
                return emit! {
                    steam_username()
                    <p class="text-error" role="alert">
                        "Complete requied Depot download first"
                    </p>
                    (button)
                };
            }

            let signin = Depot::new_signin();
            let mut sub = signin.subscribe();
            let initial = emit! {
                steam_username()
                <p>"Loading..."</p>
            }?;

            if !connected(cx) {
                return Ok(initial);
            }

            signin.start_bg();

            while let Some(event) = sub.recv_latest().await {
                match event {
                    crate::steam::DepotSigninEvent::QR(qr) => emit! {
                        <span
                            class="font-mono leading-none tracking-normal whitespace-pre text-xs"
                        >
                            (qr)
                        </span>
                        <span class="mx-auto">
                            "New QR in "
                            countdown(value: 20, live: true)
                            " seconds"
                        </span>
                        steam_username()
                    }?,
                    crate::steam::DepotSigninEvent::Failed { message } => return emit! { <p class="text-error">(message)</p> },
                    crate::steam::DepotSigninEvent::Success { steam_username } => return emit! {
                        steam_username(value: Some(steam_username.clone()))
                        <p class="text-success">
                            "Successfully signed in as "
                            (steam_username)
                        </p>
                    },
                };
            };

            emit! {
                steam_username()
                <p class="Failure">"Failed to scan QR in time"</p>
                (button)
            }
        })
    })
}

#[component]
pub async fn steam_username(
    cx: &Cx,
    #[default]
    #[into]
    value: Option<String>,
) -> Result<impl View> {
    let set_warning = signal(cx, || false);
    Ok(view! {
        <input
            type="text"
            class="invisible h-0 w-0"
            name="steam_username"
            required="true"
            @invalid=$(async |_e: Event| {
                set_warning.set(true);
            })
            if let Some(value) = value {
                value=(value)
            }
        />
        <p class="text-error" role="alert" :hidden=$(!set_warning.get())>
            "Require steam authentication"
        </p>
    })
}
