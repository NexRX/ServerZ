use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn fieldset(
    cx: &Cx,
    #[default] legend: Option<String>,
    #[default] class: &'static str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <fieldset class=(class!("fieldset", class, attrs.remove("class"))) (attrs)>
            if let Some(legend) = legend {
                fieldset_legend((legend))
            }
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
