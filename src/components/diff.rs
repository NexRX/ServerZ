use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn diff(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <figure class=(class!("diff aspect-video", attrs.remove("class"))) (attrs)>
            (child)
        </figure>
    })
}

#[component]
pub async fn diff_item_one(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("diff-item-1", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn diff_item_two(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("diff-item-2", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn diff_resizer(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            aria-label="Resize comparison"
            class=(class!("diff-resizer", attrs.remove("class")))
            (attrs)
        ></div>
    })
}
