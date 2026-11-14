use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Mask shapes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum MaskShape {
    #[default]
    #[class = "mask-squircle"]
    Squircle,
    #[class = "mask-heart"]
    Heart,
    #[class = "mask-hexagon"]
    Hexagon,
    #[class = "mask-circle"]
    Circle,
    #[class = "mask-star"]
    Star,
    #[class = "mask-triangle"]
    Triangle,
    #[class = "mask-diamond"]
    Diamond,
}

/// Mask half options.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum MaskHalf {
    #[default]
    #[class = ""]
    None,
    #[class = "mask-half-1"]
    First,
    #[class = "mask-half-2"]
    Second,
}

#[component]
pub async fn mask(
    cx: &Cx,
    #[default] shape: MaskShape,
    #[default] half: MaskHalf,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(
                "mask",
                shape.to_class(),
                half.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
