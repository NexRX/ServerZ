use crate::components::variants::{CarouselAlignment, ComponentDirection};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn carousel(
    #[default] direction: ComponentDirection,
    #[default] alignment: CarouselAlignment,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let axis = match direction {
        ComponentDirection::Horizontal => "carousel-horizontal",
        ComponentDirection::Vertical => "carousel-vertical",
    };
    Ok(
        view! { <div class=(class!("carousel", axis, format!("carousel-{}", alignment.suffix()), "scroll-smooth", attrs.remove("class"))) (attrs) tabindex="0">(child)</div> },
    )
}

#[component]
pub async fn carousel_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("carousel-item", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn carousel_slide(
    #[default] id: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div id=(id) class=(class!("carousel-item snap-center", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
#[component]
pub async fn carousel_control(
    #[default] target: String,
    label: String,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <a href=(format!("#{target}")) aria-label=(label.clone()) class=(class!("btn btn-circle", attrs.remove("class"))) (attrs)>(label)</a> },
    )
}

/// Supply indicator buttons as children, linking each button to a carousel slide ID.
#[component]
pub async fn carousel_indicators(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div role="tablist" aria-label="Carousel slides" class=(class!("join", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}

#[component]
pub async fn carousel_indicator(
    #[default] target: String,
    label: String,
    #[default] active: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let state = if active { "btn-active" } else { "" };
    Ok(
        view! { <a role="tab" href=(format!("#{target}")) aria-label=(label.clone()) aria-current=(active.then_some("true")) class=(class!("btn join-item", state, attrs.remove("class"))) (attrs)>(label)</a> },
    )
}

#[component]
pub async fn carousel_controls(
    #[default] previous: String,
    next: String,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <div role="group" aria-label="Carousel controls" class=(class!("join", attrs.remove("class"))) (attrs)><a href=(format!("#{previous}")) aria-label="Previous slide" class="btn join-item">"‹"</a><a href=(format!("#{next}")) aria-label="Next slide" class="btn join-item">"›"</a></div> },
    )
}
