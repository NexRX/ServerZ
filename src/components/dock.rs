use crate::components::variants::ComponentSize;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn dock(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <nav class=(class!("dock", attrs.remove("class"))) (attrs)>(child)</nav> })
}

#[component]
pub async fn dock_typed(
    size: ComponentSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <nav class=(class!("dock", format!("dock-{}", size.suffix()), attrs.remove("class"))) (attrs)>(child)</nav> },
    )
}

#[component]
pub async fn dock_item(
    active: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let state = if active { "dock-active" } else { "" };
    Ok(view! { <button class=(class!(state, attrs.remove("class"))) (attrs)>(child)</button> })
}

#[component]
pub async fn dock_label(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <span class=(class!("dock-label", attrs.remove("class"))) (attrs)>(child)</span> })
}
