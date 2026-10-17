use crate::components::variants::{RadialSize, RadialThickness};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn radial_progress(value: u8, #[default] mut attrs: Attributes) -> Result<impl View> {
    let value = value.min(100);
    Ok(
        view! { <div role="progressbar" aria-valuenow=(value.to_string()) style=(format!("--value:{value};")) class=(class!("radial-progress", attrs.remove("class"))) (attrs)>(format!("{value}%"))</div> },
    )
}

#[component]
pub async fn radial_progress_typed(
    value: u8,
    size: RadialSize,
    thickness: RadialThickness,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let value = value.min(100);
    Ok(
        view! { <div role="progressbar" aria-valuenow=(value.to_string()) style=(format!("--value:{value};--size:{};--thickness:{};", size.css(), thickness.css())) class=(class!("radial-progress", attrs.remove("class"))) (attrs)>(format!("{value}%"))</div> },
    )
}
