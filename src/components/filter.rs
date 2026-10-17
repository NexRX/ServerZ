use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn filter(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <form class=(class!("filter", attrs.remove("class"))) (attrs)>(child)</form> })
}

#[component]
pub async fn filter_reset(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! { <input type="reset" value="×" aria-label="Reset filter" class=(class!("btn btn-square filter-reset", attrs.remove("class"))) (attrs)> },
    )
}
