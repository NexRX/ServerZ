use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn drawer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("drawer", attrs.remove("class"))) (attrs)>(child)</div> })
}

#[component]
pub async fn drawer_typed(
    end: bool,
    open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let placement = if end { "drawer-end" } else { "" };
    let state = if open { "drawer-open" } else { "" };
    Ok(
        view! { <div class=(class!("drawer", placement, state, attrs.remove("class"))) (attrs)>(child)</div> },
    )
}

#[component]
pub async fn drawer_toggle(id: String, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! { <input id=(id) type="checkbox" class=(class!("drawer-toggle", attrs.remove("class"))) (attrs)> },
    )
}
#[component]
pub async fn drawer_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("drawer-content", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn drawer_side(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <aside class=(class!("drawer-side", attrs.remove("class"))) (attrs)>(child)</aside> },
    )
}
#[component]
pub async fn drawer_overlay(for_id: String, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! { <label for=(for_id) aria-label="Close sidebar" class=(class!("drawer-overlay", attrs.remove("class"))) (attrs)></label> },
    )
}
#[component]
pub async fn drawer_button(
    for_id: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <label for=(for_id) class=(class!("btn drawer-button", attrs.remove("class"))) (attrs)>(child)</label> },
    )
}
