use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// Input colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum InputColor {
    #[default]
    #[class = "input-neutral"]
    Neutral,
    #[class = "input-primary"]
    Primary,
    #[class = "input-secondary"]
    Secondary,
    #[class = "input-accent"]
    Accent,
    #[class = "input-info"]
    Info,
    #[class = "input-success"]
    Success,
    #[class = "input-warning"]
    Warning,
    #[class = "input-error"]
    Error,
}

/// Input sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum InputSize {
    #[class = "input-xs"]
    Xs,
    #[class = "input-sm"]
    Sm,
    #[default]
    #[class = "input-md"]
    Md,
    #[class = "input-lg"]
    Lg,
    #[class = "input-xl"]
    Xl,
}

/// Input variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum InputVariant {
    #[default]
    #[class = ""]
    Default,
    #[class = "input-ghost"]
    Ghost,
}

#[component]
pub async fn input(
    #[default] color: InputColor,
    #[default] size: InputSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <input
            class=(class!(
                "input w-full",
                color.to_class(),
                size.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
    })
}
