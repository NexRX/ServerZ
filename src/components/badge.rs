use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Badge colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum BadgeColor {
    #[default]
    #[class = "badge-neutral"]
    Neutral,
    #[class = "badge-primary"]
    Primary,
    #[class = "badge-secondary"]
    Secondary,
    #[class = "badge-accent"]
    Accent,
    #[class = "badge-info"]
    Info,
    #[class = "badge-success"]
    Success,
    #[class = "badge-warning"]
    Warning,
    #[class = "badge-error"]
    Error,
}

/// Badge sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum BadgeSize {
    #[class = "badge-xs"]
    Xs,
    #[class = "badge-sm"]
    Sm,
    #[default]
    #[class = "badge-md"]
    Md,
    #[class = "badge-lg"]
    Lg,
    #[class = "badge-xl"]
    Xl,
}

/// Badge styles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum BadgeStyle {
    #[default]
    #[class = ""]
    Solid,
    #[class = "badge-outline"]
    Outline,
    #[class = "badge-dash"]
    Dash,
    #[class = "badge-soft"]
    Soft,
    #[class = "badge-ghost"]
    Ghost,
}

#[component]
pub async fn badge(
    cx: &Cx,
    #[default] color: BadgeColor,
    #[default] size: BadgeSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            class=(class!(
                "badge",
                color.to_class(),
                size.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

#[component]
pub async fn badge_styled(
    cx: &Cx,
    color: BadgeColor,
    size: BadgeSize,
    style: BadgeStyle,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            class=(class!(
                "badge",
                color.to_class(),
                size.to_class(),
                style.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}
