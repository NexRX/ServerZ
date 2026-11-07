use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn validator(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("validator", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn validator_hint(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!("validator-hint", attrs.remove("class"))) (attrs)>(child)</p>
    })
}
