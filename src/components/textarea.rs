use crate::components::variants::{ComponentColor, ComponentSize, ComponentVariant};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn textarea(
    color: ComponentColor,
    size: ComponentSize,
    variant: ComponentVariant,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let style = if variant.suffix().is_empty() {
        String::new()
    } else {
        format!("textarea-{}", variant.suffix())
    };
    Ok(
        view! { <textarea class=(class!("textarea", format!("textarea-{}", color.suffix()), format!("textarea-{}", size.suffix()), style, attrs.remove("class"))) (attrs)></textarea> },
    )
}
