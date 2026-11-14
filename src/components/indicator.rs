use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn indicator(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("indicator", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn indicator_item(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!("indicator-item badge", attrs.remove("class"))) (attrs)>
            (child)
        </span>
    })
}
