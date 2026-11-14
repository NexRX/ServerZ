use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn fieldset(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <fieldset class=(class!("fieldset", attrs.remove("class"))) (attrs)>
            (child)
        </fieldset>
    })
}

#[component]
pub async fn fieldset_legend(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <legend class=(class!("fieldset-legend", attrs.remove("class"))) (attrs)>
            (child)
        </legend>
    })
}
