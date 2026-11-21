use crate::components::{button::button, fieldset::fieldset};
use crate::download::Downloader;
use crate::tw;
use crate::{components::progress::*, download::ReceiveLatest as _};
use derive_more::IsVariant;
use tokio::sync::broadcast::{
    Receiver,
    error::{RecvError, TryRecvError},
};
use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal, connected},
    view::{Attributes, Child, View, attributes, class, component, emit, live, view},
};

const PER_WRAPPER_CLASS: &str = tw!("flex flex-col");

#[derive(Debug, Copy, Clone, Default, IsVariant)]
pub enum DownloadProgressBtn {
    Enabled,
    Done,
    #[default]
    None,
}

#[component]
pub async fn download_progress(
    cx: &Cx,
    mut downloader: Downloader,
    start: Signal<bool>,
    #[default] fieldset: Option<(&str, &str)>,
    #[default] start_button: DownloadProgressBtn,
    #[default] show_name: bool,
) -> Result<impl View> {
    Ok(view! {
        (live! {
            let mut subscriber = downloader.subscribe();
            let has_extract = downloader.has_extract_step();
            let name = downloader.name.clone();

            if start.get() {
                downloader.start_bg();
            }

            let initial = emit! {
                download_progress_view(
                    name: &name,
                    current: 0,
                    total: 0,
                    has_extract: has_extract,
                    is_extract: false,
                    start: start.clone(),
                    completed: false,
                    fieldset: fieldset,
                    start_button: start_button,
                    show_name: show_name
                )
            }?;

            if !connected(cx) {
                Ok(initial)
            } else {
                loop {
                    let event = match subscriber.recv_latest().await {
                        Some(event) => event,
                        None => break Ok(initial),
                    };

                    match event {
                        crate::download::DownloadEvent::Downloading {
                            current,
                            total,
                            ..
                        } => emit! {
                            download_progress_view(
                                name: &name,
                                current: current,
                                total: total,
                                has_extract: has_extract,
                                is_extract: false,
                                start: start.clone(),
                                completed: false,
                                fieldset: fieldset,
                                start_button: start_button,
                                show_name: show_name
                            )
                        }?,
                        crate::download::DownloadEvent::Extracting { current, total, .. } => emit! {
                            download_progress_view(
                                name: &name,
                                current: current,
                                total: total,
                                has_extract: has_extract,
                                is_extract: true,
                                start: start.clone(),
                                completed: false,
                                fieldset: fieldset,
                                start_button: start_button,
                                show_name: show_name
                            )
                        }?,
                        crate::download::DownloadEvent::Completed { total, .. } => break Ok(
                            emit! {
                                download_progress_view(
                                    name: &name,
                                    current: total,
                                    total: total,
                                    has_extract: has_extract,
                                    is_extract: has_extract,
                                    start: start.clone(),
                                    completed: true,
                                    fieldset: fieldset,
                                    start_button: start_button,
                                    show_name: show_name
                                )
                            }?,
                        ),
                    };
                }
            }
        })
    })
}

#[component]
async fn download_progress_view(
    cx: &Cx,
    name: &str,
    current: u128,
    total: u128,
    has_extract: bool,
    is_extract: bool,
    start: Signal<bool>,
    completed: bool,
    fieldset: Option<(&str, &str)>,
    start_button: DownloadProgressBtn,
    show_name: bool,
) -> Result<impl View> {
    Ok(view! {
        if let Some((legend, label)) = fieldset {
            fieldset(
                legend: legend,
                label: label,
                bg_border: true,
                download_progress_content(
                    name: name,
                    current: current,
                    total: total,
                    has_extract: has_extract,
                    is_extract: is_extract,
                    start: start.clone(),
                    completed: completed,
                    start_button: start_button,
                    show_name: show_name
                )
            )
        } else {
            download_progress_content(
                name: name,
                current: current,
                total: total,
                has_extract: has_extract,
                is_extract: is_extract,
                start: start,
                completed: completed,
                start_button: start_button,
                show_name: show_name
            )
        }
    })
}

#[component]
async fn download_progress_content(
    cx: &Cx,
    name: &str,
    current: u128,
    total: u128,
    has_extract: bool,
    is_extract: bool,
    start: Signal<bool>,
    completed: bool,
    start_button: DownloadProgressBtn,
    show_name: bool,
) -> Result<impl View> {
    Ok(view! {
        progress_bars(
            name: name,
            current: if start_button.is_done() { 100 } else { current },
            total: if start_button.is_done() { 100 } else { total },
            has_extract: has_extract,
            is_extract: is_extract || (start_button.is_done() && has_extract),
            inline: true,
            bytes: true,
            show_name: show_name
        )
        if matches!(
            start_button,
            DownloadProgressBtn::Enabled | DownloadProgressBtn::Done,
        ) {
            button(
                attrs: attributes! {
                    class=(class!("mt-2"))
                    @click=$(async |_e: Event| {
                        start.set(true);
                    })
                    if start.get() || start_button.is_done() {
                        disabled="true"
                    }
                },
                if completed || start_button.is_done() {
                    "Done ✅"
                } else {
                    "Start"
                }
            )
        }
    })
}

fn readable_bytes(current: u128, total: u128, bits: bool) -> String {
    const BYTE_UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    const BIT_UNITS: [&str; 6] = ["b", "Kb", "Mb", "Gb", "Tb", "Pb"];

    let units = if bits { BIT_UNITS } else { BYTE_UNITS };
    let (current, total) = if bits { (current * 8, total * 8) } else { (current, total) };

    let mut size = total as f64;
    let mut unit = 0;
    while size >= 1000.0 && unit < units.len() - 1 {
        size /= 1000.0;
        unit += 1;
    }

    let divisor = 1000f64.powi(unit as i32);
    let current_scaled = current as f64 / divisor;
    let total_scaled = total as f64 / divisor;

    if unit == 0 {
        format!("{} of {} {}", current, total, units[unit])
    } else {
        format!("{:.2} of {:.2} {}", current_scaled, total_scaled, units[unit])
    }
}

#[component]
pub async fn progress_bars(
    cx: &Cx,
    name: &str,
    current: u128,
    total: u128,
    is_extract: bool,
    has_extract: bool,
    inline: bool,
    bytes: bool,
    show_name: bool,
) -> Result<impl View> {
    let current_dl = if is_extract { 100 } else { current };
    let current_ex = if is_extract { current } else { 0 };

    let total_dl = if is_extract { 100 } else { total };
    let total_ex = if is_extract { total } else { 100 };

    let is_done = if has_extract && is_extract { current_ex >= total_ex } else { current_dl >= total_dl };
    let done_txt = if is_done { "Persisted " } else { "" };

    Ok(view! {
        <div class=(PER_WRAPPER_CLASS)>
            <div>
                if show_name {
                    (&name)
                }
                if bytes && current != 100 && total != 100 {
                    <span class="text-xs font-mono text-base-content/66 ml-auto">
                        " ("
                        (done_txt)
                        (readable_bytes(current, total, false))
                        ")"
                    </span>
                }
            </div>
            progress_bar(
                label: "Download",
                value: current_dl,
                max: total_dl,
                inline: inline
            )
            if has_extract {
                progress_bar(
                    label: "Extract",
                    value: current_ex,
                    max: total_ex,
                    inline: inline
                )
            }
        </div>
    })
}

#[component]
async fn progress_bar(cx: &Cx, label: &str, value: u128, max: u128, inline: bool) -> Result<impl View> {
    Ok(view! {
        <div class="">
            <div
                class="flex flex-row justify-between items-center text-xs font-mono text-base-content/66"
            >
                <label class="whitespace-pre">(format!("{label:<9}"))</label>
                if inline {
                    progress(
                        value: value,
                        max: max,
                        attrs: attributes! { class="progress-snap" }
                    )
                }
                <label class="whitespace-pre">
                    if value == 0 {
                        (format!("{:>3}%", 0))
                    } else {
                        (format!("{:>3}%", (value * 100 / max)))
                    }
                </label>
            </div>
            if !inline {
                progress(
                    value: value,
                    max: max,
                    attrs: attributes! { class="progress-snap" }
                )
            }
        </div>
    })
}
