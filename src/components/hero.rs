use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn hero(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <section class=(class!("hero", attrs.remove("class"))) (attrs)>(child)</section> })
}
#[component]
pub async fn hero_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("hero-content text-center", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
#[component]
pub async fn hero_overlay(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! { <div class=(class!("hero-overlay", attrs.remove("class"))) (attrs)></div> })
}
