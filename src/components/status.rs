use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

/// Status colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum StatusColor {
    #[default]
    #[class = "status-neutral"]
    Neutral,
    #[class = "status-primary"]
    Primary,
    #[class = "status-secondary"]
    Secondary,
    #[class = "status-accent"]
    Accent,
    #[class = "status-info"]
    Info,
    #[class = "status-success"]
    Success,
    #[class = "status-warning"]
    Warning,
    #[class = "status-error"]
    Error,
}

/// Status sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum StatusSize {
    #[class = "status-xs"]
    Xs,
    #[class = "status-sm"]
    Sm,
    #[default]
    #[class = "status-md"]
    Md,
    #[class = "status-lg"]
    Lg,
    #[class = "status-xl"]
    Xl,
}

#[component]
pub async fn status(
    cx: &Cx,
    #[default] color: StatusColor,
    #[default] size: StatusSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            role="status"
            class=(class!(
                "status",
                color.to_class(),
                size.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        ></span>
    })
}
