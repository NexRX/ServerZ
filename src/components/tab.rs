use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Tab sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TabSize {
    #[class = "tabs-xs"]
    Xs,
    #[class = "tabs-sm"]
    Sm,
    #[default]
    #[class = "tabs-md"]
    Md,
    #[class = "tabs-lg"]
    Lg,
    #[class = "tabs-xl"]
    Xl,
}

/// Tab styles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TabStyle {
    #[default]
    #[class = ""]
    Solid,
    #[class = "tabs-border"]
    Outline,
    #[class = ""]
    Dash,
    #[class = "tabs-box"]
    Soft,
    #[class = "tabs-lift"]
    Ghost,
}

#[component]
pub async fn tabs(
    style: TabStyle,
    size: TabSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="tablist"
            class=(class!(
                "tabs",
                style.to_class(),
                size.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
