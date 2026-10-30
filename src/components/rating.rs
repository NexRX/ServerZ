use crate::components::variants::{ComponentSize, RatingMode};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn rating_star(
    #[default] name: String,
    value: u8,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <input type="radio" name=(name) value=(value.to_string()) aria-label=(format!("{value} stars")) class=(class!("mask mask-star", attrs.remove("class"))) (attrs)> },
    )
}

#[component]
pub async fn rating(
    #[default] size: ComponentSize,
    #[default] mode: RatingMode,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let mode_class = mode.class();
    Ok(
        view! { <div class=(class!("rating", format!("rating-{}", size.suffix()), mode_class, attrs.remove("class"))) (attrs)>(child)</div> },
    )
}

#[component]
pub async fn rating_reset(name: String, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! { <input type="radio" name=(name) class=(class!("rating-hidden", attrs.remove("class"))) aria-label="Clear rating"> },
    )
}

#[component]
pub async fn rating_readonly(value: u8, #[default] mut attrs: Attributes) -> Result<impl View> {
    let value = value.clamp(1, 5);
    Ok(
        view! { <div role="img" aria-label=(format!("{value} of 5 stars")) class=(class!("rating", attrs.remove("class"))) (attrs)>
            <span class=(class!("mask mask-star", (value >= 1).then_some("bg-warning")))></span>
            <span class=(class!("mask mask-star", (value >= 2).then_some("bg-warning")))></span>
            <span class=(class!("mask mask-star", (value >= 3).then_some("bg-warning")))></span>
            <span class=(class!("mask mask-star", (value >= 4).then_some("bg-warning")))></span>
            <span class=(class!("mask mask-star", (value >= 5).then_some("bg-warning")))></span>
        </div> },
    )
}
