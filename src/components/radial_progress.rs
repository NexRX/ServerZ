use crate::components::variants::{RadialSize, RadialThickness};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn radial_progress(
    #[default] value: u8,
    #[default] size: RadialSize,
    #[default] thickness: RadialThickness,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let value = value.min(100);
    Ok(
        view! { <div role="progressbar" aria-valuenow=(value.to_string()) style=(format!("--value:{value};--size:{};--thickness:{};", size.css(), thickness.css())) class=(class!("radial-progress", attrs.remove("class"))) (attrs)>(format!("{value}%"))</div> },
    )
}
