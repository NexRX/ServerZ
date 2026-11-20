use crate::download::Downloader;
use crate::tw;
use crate::{components::progress::*, download::ReceiveLatest as _};
use tokio::sync::broadcast::{
    Receiver,
    error::{RecvError, TryRecvError},
};
use topcoat::{
    Result,
    context::Cx,
    runtime::{Signal, connected},
    view::{Attributes, Child, View, class, component, emit, live, view},
};

const PER_WRAPPER_CLASS: &str = tw!("flex flex-col");

#[component]
pub async fn download_progress(cx: &Cx, mut downloader: Downloader, start: Signal<bool>) -> Result<impl View> {
    Ok(view! {
        (live! {
            let mut subscriber = downloader.subscribe();
            let has_extract = downloader.has_extract_step();
            let name = downloader.name.clone();

            if start.get() {
                downloader.start_bg();
            }

            let initial = emit! {
                progress_wrapper(
                    name: (&name),
                    current: 0,
                    total: 0,
                    has_extract: has_extract,
                    is_extract: false,
                    inline: true,
                    bytes: true
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
                            progress_wrapper(
                                name: &name,
                                current: current,
                                total: total,
                                has_extract: has_extract,
                                is_extract: false,
                                inline: true,
                                bytes: true
                            )
                        }?,
                        crate::download::DownloadEvent::Extracting { current, total, .. } => emit! {
                            progress_wrapper(
                                name: &name,
                                current: current,
                                total: total,
                                has_extract: has_extract,
                                is_extract: true,
                                inline: true,
                                bytes: true
                            )
                        }?,
                        crate::download::DownloadEvent::Completed { total, .. } => break Ok(
                            emit! {
                                progress_wrapper(
                                    name: &name,
                                    current: total,
                                    total: total,
                                    has_extract: has_extract,
                                    is_extract: has_extract,
                                    inline: true,
                                    bytes: true
                                )
                            }?,
                        ),
                    };
                }
            }
        })
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
pub async fn progress_wrapper(
    cx: &Cx,
    name: &str,
    current: u128,
    total: u128,
    is_extract: bool,
    has_extract: bool,
    inline: bool,
    bytes: bool,
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
                (&name)
                if bytes {
                    <span class="text-xs font-mono text-base-content/66 ml-auto">
                        " ("
                        (done_txt)
                        (readable_bytes(current, total, false))
                        ")"
                    </span>
                }
            </div>
            inner_progress(
                label: "Download",
                value: current_dl,
                max: total_dl,
                inline: inline
            )
            if has_extract {
                inner_progress(
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
async fn inner_progress(cx: &Cx, label: &str, value: u128, max: u128, inline: bool) -> Result<impl View> {
    Ok(view! {
        <div class="">
            <div
                class="flex flex-row justify-between items-center text-xs font-mono text-base-content/66"
            >
                <label class="whitespace-pre">(format!("{label:<9}"))</label>
                if inline {
                    progress(value: value, max: max)
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
                progress(value: value, max: max)
            }
        </div>
    })
}
