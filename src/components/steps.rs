use crate::components::variants::{ComponentColor, ComponentDirection};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn steps(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <ul class=(class!("steps", attrs.remove("class"))) (attrs)>(child)</ul> })
}

#[component]
pub async fn steps_typed(
    direction: ComponentDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <ul class=(class!("steps", format!("steps-{}", direction.suffix()), attrs.remove("class"))) (attrs)>(child)</ul> },
    )
}

#[component]
pub async fn step(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <li class=(class!("step", attrs.remove("class"))) (attrs)>(child)</li> })
}

#[component]
pub async fn step_typed(
    color: ComponentColor,
    content: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <li data-content=(content) class=(class!("step", format!("step-{}", color.suffix()), attrs.remove("class"))) (attrs)>(child)</li> },
    )
}

#[component]
pub async fn step_icon(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <span class=(class!("step-icon", attrs.remove("class"))) (attrs)>(child)</span> })
}
