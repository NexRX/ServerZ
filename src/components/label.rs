use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn label(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label class=(class!("label", attrs.remove("class"))) (attrs)>(child)</label>
    })
}

#[component]
pub async fn floating_label(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <label class=(class!("floating-label", attrs.remove("class"))) (attrs)>
                (child)
            </label>
        },
    )
}
