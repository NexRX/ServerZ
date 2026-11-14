use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn diff(
    cx: &Cx,
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
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("diff-item-1", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn diff_item_two(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("diff-item-2", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn diff_resizer(cx: &Cx, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            aria-label="Resize comparison"
            class=(class!("diff-resizer", attrs.remove("class")))
            (attrs)
        ></div>
    })
}
