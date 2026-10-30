use crate::components::variants::{ComponentColor, ComponentSize};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn checkbox(
    #[default] color: ComponentColor,
    #[default] size: ComponentSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <input type="checkbox" class=(class!("checkbox", format!("checkbox-{}", color.suffix()), format!("checkbox-{}", size.suffix()), attrs.remove("class"))) (attrs)> },
    )
}
