use crate::components::variants::{ComponentColor, ComponentDirection};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn divider(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("divider", attrs.remove("class"))) (attrs)>(child)</div> })
}

#[component]
pub async fn divider_typed(
    color: ComponentColor,
    direction: ComponentDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("divider", format!("divider-{}", color.suffix()), format!("divider-{}", direction.suffix()), attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
