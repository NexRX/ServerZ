use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// Checkbox colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum CheckboxColor {
    #[default]
    #[class = "checkbox-neutral"]
    Neutral,
    #[class = "checkbox-primary"]
    Primary,
    #[class = "checkbox-secondary"]
    Secondary,
    #[class = "checkbox-accent"]
    Accent,
    #[class = "checkbox-info"]
    Info,
    #[class = "checkbox-success"]
    Success,
    #[class = "checkbox-warning"]
    Warning,
    #[class = "checkbox-error"]
    Error,
}

/// Checkbox sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum CheckboxSize {
    #[class = "checkbox-xs"]
    Xs,
    #[class = "checkbox-sm"]
    Sm,
    #[default]
    #[class = "checkbox-md"]
    Md,
    #[class = "checkbox-lg"]
    Lg,
    #[class = "checkbox-xl"]
    Xl,
}

#[component]
pub async fn checkbox(
    #[default] color: CheckboxColor,
    #[default] size: CheckboxSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <input
            type="checkbox"
            class=(class!(
                "checkbox",
                color.to_class(),
                size.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
    })
}
