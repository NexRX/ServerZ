use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// Radio colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum RadioColor {
    #[default]
    #[class = "radio-neutral"]
    Neutral,
    #[class = "radio-primary"]
    Primary,
    #[class = "radio-secondary"]
    Secondary,
    #[class = "radio-accent"]
    Accent,
    #[class = "radio-info"]
    Info,
    #[class = "radio-success"]
    Success,
    #[class = "radio-warning"]
    Warning,
    #[class = "radio-error"]
    Error,
}

/// Radio sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum RadioSize {
    #[class = "radio-xs"]
    Xs,
    #[class = "radio-sm"]
    Sm,
    #[default]
    #[class = "radio-md"]
    Md,
    #[class = "radio-lg"]
    Lg,
    #[class = "radio-xl"]
    Xl,
}

#[component]
pub async fn radio(
    name: String,
    color: RadioColor,
    size: RadioSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <input
            type="radio"
            name=(name)
            class=(class!(
                "radio",
                color.to_class(),
                size.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
    })
}
