use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Alert colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum AlertColor {
    #[default]
    #[class = "alert-neutral"]
    Neutral,
    #[class = "alert-primary"]
    Primary,
    #[class = "alert-secondary"]
    Secondary,
    #[class = "alert-accent"]
    Accent,
    #[class = "alert-info"]
    Info,
    #[class = "alert-success"]
    Success,
    #[class = "alert-warning"]
    Warning,
    #[class = "alert-error"]
    Error,
}

/// Alert styles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum AlertStyle {
    #[default]
    #[class = ""]
    Solid,
    #[class = "alert-outline"]
    Outline,
    #[class = "alert-dash"]
    Dash,
    #[class = "alert-soft"]
    Soft,
    #[class = "alert-ghost"]
    Ghost,
}

/// Alert direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum AlertDirection {
    #[default]
    #[class = "alert-horizontal"]
    Horizontal,
    #[class = "alert-vertical"]
    Vertical,
}

#[component]
pub async fn alert(
    cx: &Cx,
    #[default] color: AlertColor,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="alert"
            class=(class!("alert", color.to_class(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

#[component]
pub async fn alert_styled(
    cx: &Cx,
    color: AlertColor,
    style: AlertStyle,
    direction: AlertDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="alert"
            class=(class!(
                "alert",
                color.to_class(),
                style.to_class(),
                direction.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
