use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn text_rotate(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!("text-rotate", attrs.remove("class"))) (attrs)>
            <span>(child)</span>
        </span>
    })
}
