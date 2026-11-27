use crate::download::HttpDownloader;
use crate::tw;
use crate::{components::progress::*, download::ReceiveLatest as _};
use crate::{
    components::{
        button::{ButtonType, button},
        fieldset::fieldset,
    },
    download::Downloader,
};
use derive_more::IsVariant;
use tokio::sync::broadcast::{
    Receiver,
    error::{RecvError, TryRecvError},
};
use topcoat::runtime::signal;
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
    downloader: impl Downloader + Send,
    start: Signal<bool>,
    #[default]
    #[into]
    fieldset: Option<(&str, &str)>,
    #[default] start_button: DownloadProgressBtn,
    #[default] show_name: bool,
    #[default]
    #[into]
    form_marker: Option<&str>,
) -> Result<impl View> {
    let form_warning = signal(cx, || false);
    let started = start.get();

    Ok(view! {
        (live! {
            let mut subscriber = downloader.subscribe();
            let has_extract = downloader.has_extract_step();
            let name = downloader.name();

            if started {
                downloader.start_bg();
            }

            let initial = emit! {
                download_progress_view(
                    name: &name,
                    current: 0f32,
                    total: 0f32,
                    has_extract: has_extract,
                    is_extract: false,
                    start: start.clone(),
                    completed: false,
                    fieldset: fieldset,
                    start_button: start_button,
                    show_name: show_name,
                    form_marker: form_marker,
                    form_warning: form_warning.clone()
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
                        crate::download::DownloadEvent::Failed { name, message } => break emit! {
                            <p class="text-error" role="alert">
                                (crate::download::DownloadEvent::Failed { name, message }.to_string(

                                ))
                            </p>
                        },
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
                                show_name: show_name,
                                form_marker: form_marker,
                                form_warning: form_warning.clone()
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
                                show_name: show_name,
                                form_marker: form_marker,
                                form_warning: form_warning.clone()
                            )
                        }?,
                        crate::download::DownloadEvent::Completed { total, .. } => break emit! {
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
                                show_name: show_name,
                                form_marker: form_marker,
                                form_warning: form_warning.clone()
                            )
                        },
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
    current: f32,
    total: f32,
    has_extract: bool,
    is_extract: bool,
    start: Signal<bool>,
    completed: bool,
    fieldset: Option<(&str, &str)>,
    start_button: DownloadProgressBtn,
    show_name: bool,
    form_marker: Option<&str>,
    form_warning: Signal<bool>,
) -> Result<impl View> {
    Ok(view! {
        if let Some((legend, label)) = fieldset {
            fieldset(
                legend: legend,
                label: label,
                bordered: true,
                error: form_warning.get(),
                class: "w-full",
                download_progress_content(
                    name: name,
                    current: current,
                    total: total,
                    has_extract: has_extract,
                    is_extract: is_extract,
                    start: start.clone(),
                    completed: completed,
                    start_button: start_button,
                    show_name: show_name,
                    form_marker: form_marker,
                    form_warning: form_warning.clone()
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
                show_name: show_name,
                form_marker: form_marker,
                form_warning: form_warning
            )
        }
    })
}

#[component]
async fn download_progress_content(
    cx: &Cx,
    name: &str,
    current: f32,
    total: f32,
    has_extract: bool,
    is_extract: bool,
    start: Signal<bool>,
    completed: bool,
    start_button: DownloadProgressBtn,
    show_name: bool,
    form_marker: Option<&str>,
    form_warning: Signal<bool>,
) -> Result<impl View> {
    Ok(view! {
        progress_bars(
            name: name,
            current: if start_button.is_done() { 100f32 } else { current },
            total: if start_button.is_done() { 100f32 } else { total },
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
                ty: ButtonType::Button,
                attrs: attributes! {
                    class=(class!("mt-2"))
                    @click=$(async |_e: Event| {
                        form_warning.set(false);
                        start.set(true);
                    })
                    if start.get() || start_button.is_done() {
                        disabled="true"
                    }
                },
                if completed || start_button.is_done() {
                    "Done "
                    if let Some(form_marker) = form_marker {
                        <input
                            type="checkbox"
                            class="checkbox checkbox-success checkbox-sm"
                            checked="checked"
                            value="true"
                            name=(form_marker)
                        />
                    } else {
                        "✅"
                    }
                } else {
                    "Start"
                    if let Some(form_marker) = form_marker {
                        <input
                            type="checkbox"
                            class="invisible"
                            name=(form_marker)
                            required="true"
                            @invalid=$(async |_e: Event| {
                                form_warning.set(true);
                            })
                        />
                    }
                }
            )
            if form_warning.get() {
                <p class="text-error text-sm" role="alert">
                    "Start & finish download before continuing"
                </p>
            }
        }
    })
}

fn readable_bytes(current: f32, total: f32, bits: bool) -> String {
    const BYTE_UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    const BIT_UNITS: [&str; 6] = ["b", "Kb", "Mb", "Gb", "Tb", "Pb"];

    let units = if bits { BIT_UNITS } else { BYTE_UNITS };
    let (current, total) = if bits { (current * 8f32, total * 8f32) } else { (current, total) };

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
    current: f32,
    total: f32,
    is_extract: bool,
    has_extract: bool,
    inline: bool,
    bytes: bool,
    show_name: bool,
) -> Result<impl View> {
    let current_dl = if is_extract { 100f32 } else { current };
    let current_ex = if is_extract { current } else { 0f32 };

    let total_dl = if is_extract { 100f32 } else { total };
    let total_ex = if is_extract { total } else { 100f32 };

    let is_done = if has_extract && is_extract { current_ex >= total_ex } else { current_dl >= total_dl };
    let done_txt = if is_done { "Persisted " } else { "" };

    Ok(view! {
        <div class=(PER_WRAPPER_CLASS)>
            <div>
                if show_name {
                    (&name)
                }
                if bytes && current != 100f32 && total != 100f32 {
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
async fn progress_bar(cx: &Cx, label: &str, value: f32, max: f32, inline: bool) -> Result<impl View> {
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
                    if value == 0f32 {
                        (format!("{:>3}%", 0))
                    } else {
                        (format!("{:>3}%", (value * 100f32 / max)))
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
