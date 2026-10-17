use crate::components::variants::{ComponentDirection, ComponentStyle};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn timeline(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <ul class=(class!("timeline", attrs.remove("class"))) (attrs)>(child)</ul> })
}
#[component]
pub async fn timeline_typed(
    direction: ComponentDirection,
    style: ComponentStyle,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let s = match style {
        ComponentStyle::Solid => "",
        ComponentStyle::Outline => "timeline-box",
        ComponentStyle::Dash => "timeline-compact",
        ComponentStyle::Soft => "timeline-snap-icon",
        ComponentStyle::Ghost => "",
    };
    Ok(
        view! { <ul class=(class!("timeline", format!("timeline-{}", direction.suffix()), s, attrs.remove("class"))) (attrs)>(child)</ul> },
    )
}
#[component]
pub async fn timeline_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <li class=(class!(attrs.remove("class"))) (attrs)>(child)</li> })
}
#[component]
pub async fn timeline_start(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("timeline-start", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn timeline_middle(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("timeline-middle", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
#[component]
pub async fn timeline_end(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("timeline-end", attrs.remove("class"))) (attrs)>(child)</div> })
}
