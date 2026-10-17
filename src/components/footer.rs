use crate::components::variants::ComponentDirection;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <footer class=(class!("footer", attrs.remove("class"))) (attrs)>(child)</footer> })
}

#[component]
pub async fn footer_typed(
    direction: ComponentDirection,
    center: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let placement = if center { "footer-center" } else { "" };
    Ok(
        view! { <footer class=(class!("footer", format!("footer-{}", direction.suffix()), placement, attrs.remove("class"))) (attrs)>(child)</footer> },
    )
}

#[component]
pub async fn footer_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <h6 class=(class!("footer-title", attrs.remove("class"))) (attrs)>(child)</h6> })
}
