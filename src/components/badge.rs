use crate::components::variants::{ComponentColor, ComponentSize, ComponentStyle};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn badge(
    #[default] color: ComponentColor,
    #[default] size: ComponentSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <span class=(class!("badge", format!("badge-{}", color.suffix()), format!("badge-{}", size.suffix()), attrs.remove("class"))) (attrs)>(child)</span> },
    )
}

#[component]
pub async fn badge_styled(
    color: ComponentColor,
    size: ComponentSize,
    style: ComponentStyle,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let style_class = if style.suffix().is_empty() {
        String::new()
    } else {
        format!("badge-{}", style.suffix())
    };
    Ok(
        view! { <span class=(class!("badge", format!("badge-{}", color.suffix()), format!("badge-{}", size.suffix()), style_class, attrs.remove("class"))) (attrs)>(child)</span> },
    )
}
