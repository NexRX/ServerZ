use crate::components::variants::ComponentDirection;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn join(
    #[default] direction: ComponentDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("join", format!("join-{}", direction.suffix()), attrs.remove("class"))) (attrs)>(child)</div> },
    )
}

#[component]
pub async fn join_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("join-item", attrs.remove("class"))) (attrs)>(child)</div> })
}
