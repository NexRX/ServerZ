use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Divider colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum DividerColor {
    #[default]
    #[class = "divider-neutral"]
    Neutral,
    #[class = "divider-primary"]
    Primary,
    #[class = "divider-secondary"]
    Secondary,
    #[class = "divider-accent"]
    Accent,
    #[class = "divider-info"]
    Info,
    #[class = "divider-success"]
    Success,
    #[class = "divider-warning"]
    Warning,
    #[class = "divider-error"]
    Error,
}

/// Divider direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum DividerDirection {
    #[default]
    #[class = "divider-horizontal"]
    Horizontal,
    #[class = "divider-vertical"]
    Vertical,
}

#[component]
pub async fn divider(
    #[default] color: DividerColor,
    #[default(DividerDirection::Vertical)] direction: DividerDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(
                "divider",
                color.to_class(),
                direction.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
