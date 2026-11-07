use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// Textarea colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TextareaColor {
    #[default]
    #[class = "textarea-neutral"]
    Neutral,
    #[class = "textarea-primary"]
    Primary,
    #[class = "textarea-secondary"]
    Secondary,
    #[class = "textarea-accent"]
    Accent,
    #[class = "textarea-info"]
    Info,
    #[class = "textarea-success"]
    Success,
    #[class = "textarea-warning"]
    Warning,
    #[class = "textarea-error"]
    Error,
}

/// Textarea sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TextareaSize {
    #[class = "textarea-xs"]
    Xs,
    #[class = "textarea-sm"]
    Sm,
    #[default]
    #[class = "textarea-md"]
    Md,
    #[class = "textarea-lg"]
    Lg,
    #[class = "textarea-xl"]
    Xl,
}

/// Textarea variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TextareaVariant {
    #[default]
    #[class = ""]
    Default,
    #[class = "textarea-ghost"]
    Ghost,
}

#[component]
pub async fn textarea(
    color: TextareaColor,
    size: TextareaSize,
    variant: TextareaVariant,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <textarea
            class=(class!(
                "textarea",
                color.to_class(),
                size.to_class(),
                variant.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        ></textarea>
    })
}
