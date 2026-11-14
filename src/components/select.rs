use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Select colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum SelectColor {
    #[default]
    #[class = "select-neutral"]
    Neutral,
    #[class = "select-primary"]
    Primary,
    #[class = "select-secondary"]
    Secondary,
    #[class = "select-accent"]
    Accent,
    #[class = "select-info"]
    Info,
    #[class = "select-success"]
    Success,
    #[class = "select-warning"]
    Warning,
    #[class = "select-error"]
    Error,
}

/// Select sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum SelectSize {
    #[class = "select-xs"]
    Xs,
    #[class = "select-sm"]
    Sm,
    #[default]
    #[class = "select-md"]
    Md,
    #[class = "select-lg"]
    Lg,
    #[class = "select-xl"]
    Xl,
}

/// Select variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum SelectVariant {
    #[default]
    #[class = ""]
    Default,
    #[class = "select-ghost"]
    Ghost,
}

#[component]
pub async fn select(
    cx: &Cx,
    color: SelectColor,
    size: SelectSize,
    variant: SelectVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <select
            class=(class!(
                "select",
                color.to_class(),
                size.to_class(),
                variant.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </select>
    })
}
