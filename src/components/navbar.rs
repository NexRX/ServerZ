use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn navbar(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <nav class=(class!("navbar", attrs.remove("class"))) (attrs)>(child)</nav> })
}

#[component]
pub async fn navbar_start(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("navbar-start", attrs.remove("class"))) (attrs)>(child)</div> })
}

#[component]
pub async fn navbar_center(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("navbar-center", attrs.remove("class"))) (attrs)>(child)</div> })
}

#[component]
pub async fn navbar_end(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("navbar-end", attrs.remove("class"))) (attrs)>(child)</div> })
}
