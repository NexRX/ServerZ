use crate::components::variants::ComponentColor;
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn progress(
    value: u8,
    #[default] max: u8,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <progress value=(value.min(max)) max=(max.max(1)) class=(class!("progress", attrs.remove("class"))) (attrs)></progress> },
    )
}

#[component]
pub async fn progress_typed(
    value: u8,
    max: u8,
    color: ComponentColor,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <progress value=(value.min(max)) max=(max.max(1)) class=(class!("progress", format!("progress-{}", color.suffix()), attrs.remove("class"))) (attrs)></progress> },
    )
}
