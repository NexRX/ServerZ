use crate::components::variants::{ComponentColor, ComponentDirection, ComponentSize};
use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn range(
    #[default] color: ComponentColor,
    #[default] size: ComponentSize,
    #[default] direction: ComponentDirection,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let direction_class = if matches!(direction, ComponentDirection::Vertical) {
        "range-vertical"
    } else {
        ""
    };
    Ok(
        view! { <input type="range" min="0" max="100" class=(class!("range", format!("range-{}", color.suffix()), format!("range-{}", size.suffix()), direction_class, attrs.remove("class"))) (attrs)> },
    )
}
