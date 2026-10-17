use crate::components::variants::MegamenuLayout;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn megamenu(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <nav class=(class!("megamenu", attrs.remove("class"))) (attrs)>(child)</nav> })
}

#[component]
pub async fn megamenu_typed(
    layout: MegamenuLayout,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <nav class=(class!("megamenu", layout.class(), attrs.remove("class"))) (attrs)>(child)</nav> },
    )
}

#[component]
pub async fn megamenu_active(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! { <span class=(class!("megamenu-active", attrs.remove("class"))) (attrs)></span> })
}

#[component]
pub async fn megamenu_item(
    id: String,
    label: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div><button popovertarget=(id.clone()) class=(class!(attrs.remove("class"))) (attrs)>(label)</button><div id=(id) popover="popover">(child)</div></div> },
    )
}
