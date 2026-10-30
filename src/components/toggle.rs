use crate::components::variants::{ComponentColor, ComponentSize};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn toggle(
    #[default] color: ComponentColor,
    #[default] size: ComponentSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <input type="checkbox" class=(class!("toggle", format!("toggle-{}", color.suffix()), format!("toggle-{}", size.suffix()), attrs.remove("class"))) (attrs)> },
    )
}
