use crate::components::variants::{ComponentColor, ComponentSize};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn radio(name: String, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! { <input type="radio" name=(name) class=(class!("radio", attrs.remove("class"))) (attrs)> },
    )
}

#[component]
pub async fn radio_typed(
    name: String,
    color: ComponentColor,
    size: ComponentSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <input type="radio" name=(name) class=(class!("radio", format!("radio-{}", color.suffix()), format!("radio-{}", size.suffix()), attrs.remove("class"))) (attrs)> },
    )
}
