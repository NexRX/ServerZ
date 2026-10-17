use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn kbd(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <kbd class=(class!("kbd", attrs.remove("class"))) (attrs)>(child)</kbd> })
}
