use crate::components::variants::{ComponentColor, ComponentSize, ComponentVariant};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn select(
    color: ComponentColor,
    size: ComponentSize,
    variant: ComponentVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let style = if variant.suffix().is_empty() {
        String::new()
    } else {
        format!("select-{}", variant.suffix())
    };
    Ok(
        view! { <select class=(class!("select", format!("select-{}", color.suffix()), format!("select-{}", size.suffix()), style, attrs.remove("class"))) (attrs)>(child)</select> },
    )
}
