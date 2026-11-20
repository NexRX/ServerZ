use crate::components::prelude::*;
use crate::guard::Guard;
use crate::steam::{Depot, Steam};
use tokio::sync::broadcast::error::RecvError;
use topcoat::runtime::{Event, signal};
use topcoat::view::component;
use topcoat::view::{emit, live};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};
use tracing::debug;

#[page]
pub async fn setup_steam(cx: &Cx) -> Result<impl View> {
    Steam::ensure_unavailable(cx).await?;

    let started = signal(cx, || false);

    Ok(view! {
        if started.get() {
            download_progress()
        } else {
            "Start Steam Download?"
            button(
                attrs: attributes! {
                    @click=$(async |_e: Event| {
                        started.set(true);
                    })
                },
                "Start"
            )
        }
    })
}

#[component]
async fn download_progress(cx: &Cx) -> Result<impl View> {
    debug!("Starting download");
    Ok(view! {
        <h1>"Downloader..."</h1>
        (live! {
            let mut downloader = Depot::downloader();
            let mut subscriber = downloader.subscribe();
            downloader.start_bg();

            loop {
                let event = match subscriber.recv().await {
                    Ok(event) => event,
                    Err(RecvError::Lagged(_)) => continue,
                    Err(RecvError::Closed) => break,
                };
                emit! { <p>(event.to_string())</p> }?;
                if event.is_completed() {
                    break;
                }
            };
            emit! { <p>"Done!"</p> }
        })
    })
}
