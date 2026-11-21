use crate::components::prelude::*;
use crate::guard::Guard;
use crate::steam::{Depot, Steam};
use topcoat::runtime::signal;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

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
    })
}
