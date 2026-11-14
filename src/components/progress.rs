use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

/// Progress colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum ProgressColor {
    #[default]
    #[class = "progress-neutral"]
    Neutral,
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

#[component]
pub async fn progress(
    cx: &Cx,
    #[default] value: u8,
    max: u8,
    #[default] color: ProgressColor,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <progress
            value=(value.min(max))
            max=(max.max(1))
            class=(class!("progress", color.to_class(), attrs.remove("class")))
            (attrs)
        ></progress>
    })
}
