use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn skeleton(cx: &Cx, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            aria-label="Loading"
            class=(class!("skeleton", attrs.remove("class")))
            (attrs)
        ></div>
    })
}
