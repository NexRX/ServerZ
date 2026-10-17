use crate::components::variants::{ComponentSize, LoadingStyle};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn loading(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! { <span role="status" aria-label="Loading" class=(class!("loading loading-spinner", attrs.remove("class"))) (attrs)></span> },
    )
}

#[component]
pub async fn loading_typed(
    style: LoadingStyle,
    size: ComponentSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <span role="status" aria-label="Loading" class=(class!("loading", format!("loading-{}", style.suffix()), format!("loading-{}", size.suffix()), attrs.remove("class"))) (attrs)></span> },
    )
}
