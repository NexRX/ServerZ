use crate::components::variants::{ComponentColor, ComponentDirection, ComponentStyle};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn alert(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div role="alert" class=(class!("alert", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}

#[component]
pub async fn alert_typed(
    color: ComponentColor,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div role="alert" class=(class!("alert", format!("alert-{}", color.suffix()), attrs.remove("class"))) (attrs)>(child)</div> },
    )
}

#[component]
pub async fn alert_styled(
    color: ComponentColor,
    style: ComponentStyle,
    direction: ComponentDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let style_class = if style.suffix().is_empty() {
        String::new()
    } else {
        format!("alert-{}", style.suffix())
    };
    Ok(
        view! { <div role="alert" class=(class!("alert", format!("alert-{}", color.suffix()), style_class, format!("alert-{}", direction.suffix()), attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
