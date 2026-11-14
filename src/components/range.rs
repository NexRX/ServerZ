use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Range colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum RangeColor {
    #[default]
    #[class = "range-neutral"]
    Neutral,
    #[class = "range-primary"]
    Primary,
    #[class = "range-secondary"]
    Secondary,
    #[class = "range-accent"]
    Accent,
    #[class = "range-info"]
    Info,
    #[class = "range-success"]
    Success,
    #[class = "range-warning"]
    Warning,
    #[class = "range-error"]
    Error,
}

/// Range sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum RangeSize {
    #[class = "range-xs"]
    Xs,
    #[class = "range-sm"]
    Sm,
    #[default]
    #[class = "range-md"]
    Md,
    #[class = "range-lg"]
    Lg,
    #[class = "range-xl"]
    Xl,
}

/// Range directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum RangeDirection {
    #[default]
    #[class = ""]
    Horizontal,
    #[class = "range-vertical"]
    Vertical,
}

#[component]
pub async fn range(
    cx: &Cx,
    #[default] color: RangeColor,
    #[default] size: RangeSize,
    #[default] direction: RangeDirection,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <input
            type="range"
            min="0"
            max="100"
            class=(class!(
                "range",
                color.to_class(),
                size.to_class(),
                direction.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
    })
}
