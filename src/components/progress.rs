use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

/// Progress colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum ProgressColor {
    #[class = "progress-neutral"]
    Neutral,
    #[default]
    #[class = "progress-primary"]
    Primary,
    #[class = "progress-secondary"]
    Secondary,
    #[class = "progress-accent"]
    Accent,
    #[class = "progress-info"]
    Info,
    #[class = "progress-success"]
    Success,
    #[class = "progress-warning"]
    Warning,
    #[class = "progress-error"]
    Error,
}

fn percentage(value: impl ToString, max: impl ToString) -> f64 {
    let value = value.to_string().parse::<f64>().unwrap_or(0.0);
    let max = max.to_string().parse::<f64>().unwrap_or(0.0);

    if !value.is_finite() || !max.is_finite() || max <= 0.0 {
        return 0.0;
    }

    (value / max * 100.0).clamp(0.0, 100.0)
}

#[component]
pub async fn progress(
    cx: &Cx,
    value: impl ToString + Send,
    max: impl ToString + Send,
    #[default] color: ProgressColor,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let value = percentage(value, max);

    Ok(view! {
        <progress
            value=(value)
            max=(100)
            class=(class!("progress", color.to_class(), attrs.remove("class")))
            (attrs)
        ></progress>
    })
}
