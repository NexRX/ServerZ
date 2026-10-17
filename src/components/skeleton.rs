use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn skeleton(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! { <div aria-label="Loading" class=(class!("skeleton", attrs.remove("class"))) (attrs)></div> },
    )
}
