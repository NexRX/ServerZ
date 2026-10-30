use crate::components::variants::{ComponentSize, LoadingStyle};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn loading(
    #[default] style: LoadingStyle,
    #[default] size: ComponentSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <span role="status" aria-label="Loading" class=(class!("loading", format!("loading-{}", style.suffix()), format!("loading-{}", size.suffix()), attrs.remove("class"))) (attrs)></span> },
    )
}
