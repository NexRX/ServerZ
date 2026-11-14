use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Rating sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum RatingSize {
    #[class = "rating-xs"]
    Xs,
    #[class = "rating-sm"]
    Sm,
    #[default]
    #[class = "rating-md"]
    Md,
    #[class = "rating-lg"]
    Lg,
    #[class = "rating-xl"]
    Xl,
}

/// Rating modes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum RatingMode {
    #[default]
    #[class = ""]
    Full,
    #[class = "rating-half"]
    Half,
    #[class = ""]
    ReadOnly,
}

#[component]
pub async fn rating(
    cx: &Cx,
    #[default] size: RatingSize,
    #[default] mode: RatingMode,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(
                "rating",
                size.to_class(),
                mode.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
