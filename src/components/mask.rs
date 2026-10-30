use crate::components::variants::{MaskHalf, MaskShape};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn mask(
    #[default] shape: MaskShape,
    #[default] half: MaskHalf,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("mask", format!("mask-{}", shape.suffix()), half.class(), attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
