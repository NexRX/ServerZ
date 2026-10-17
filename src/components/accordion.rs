use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn accordion(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <section class=(class!(attrs.remove("class"))) (attrs)>(child)</section> })
}

/// Wraps one accordion group. Pass the same `name` to each accordion item.
#[component]
pub async fn accordion_group(
    name: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <section data-accordion-group=(name) class=(class!(attrs.remove("class"))) (attrs)>(child)</section> },
    )
}

#[component]
pub async fn accordion_item(
    title: String,
    group: String,
    #[default] open: bool,
    #[default] plus: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let modifier = if plus {
        "collapse-plus"
    } else {
        "collapse-arrow"
    };
    let checked = open.then_some("checked");
    Ok(
        view! { <div class=(class!("collapse", modifier, "border border-base-300", attrs.remove("class"))) (attrs)><input type="radio" name=(group) checked=(checked)><div class="collapse-title font-semibold">(title)</div><div class="collapse-content">(child)</div></div> },
    )
}
