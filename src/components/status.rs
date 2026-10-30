use crate::components::variants::{ComponentColor, ComponentSize};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn status(
    #[default] color: ComponentColor,
    #[default] size: ComponentSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <span role="status" class=(class!("status", format!("status-{}", color.suffix()), format!("status-{}", size.suffix()), attrs.remove("class"))) (attrs)></span> },
    )
}
