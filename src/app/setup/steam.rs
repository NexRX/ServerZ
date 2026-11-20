use crate::components::prelude::*;
use crate::guard::Guard;
use crate::steam::{Depot, Steam};
use topcoat::runtime::{Event, signal};
use topcoat::view::{emit, live};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};

#[page]
pub async fn setup_steam(cx: &Cx) -> Result<impl View> {
    Steam::ensure_unavailable(cx).await?;

    let start = signal(cx, || false);

    Ok(view! {
        fieldset(
            legend: Some("Required Download: Depot"),
            label: Some("This tool is required to download items from steam"),
            download_progress(downloader: Depot::downloader(), start: start.clone())
            button(
                attrs: attributes! {
                    @click=$(async |_e: Event| {
                        start.set(true);
                    })
                    if start.get() {
                        disabled="true"
                    }
                },
                "Start"
            )
        )
    })
}
