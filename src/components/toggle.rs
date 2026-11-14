use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

/// Toggle colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum ToggleColor {
    #[default]
    #[class = "toggle-neutral"]
    Neutral,
    #[class = "toggle-primary"]
    Primary,
    #[class = "toggle-secondary"]
    Secondary,
    #[class = "toggle-accent"]
    Accent,
    #[class = "toggle-info"]
    Info,
    #[class = "toggle-success"]
    Success,
    #[class = "toggle-warning"]
    Warning,
    #[class = "toggle-error"]
    Error,
}

/// Toggle sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum ToggleSize {
    #[class = "toggle-xs"]
    Xs,
    #[class = "toggle-sm"]
    Sm,
    #[default]
    #[class = "toggle-md"]
    Md,
    #[class = "toggle-lg"]
    Lg,
    #[class = "toggle-xl"]
    Xl,
}

#[component]
pub async fn toggle(
    cx: &Cx,
    #[default] color: ToggleColor,
    #[default] size: ToggleSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <input
            type="checkbox"
            class=(class!(
                "toggle",
                color.to_class(),
                size.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
    })
}
