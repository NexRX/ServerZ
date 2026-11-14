use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Timeline directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TimelineDirection {
    #[default]
    #[class = "timeline-horizontal"]
    Horizontal,
    #[class = "timeline-vertical"]
    Vertical,
}

/// Timeline styles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TimelineStyle {
    #[default]
    #[class = ""]
    Solid,
    #[class = "timeline-box"]
    Outline,
    #[class = "timeline-compact"]
    Dash,
    #[class = "timeline-snap-icon"]
    Soft,
    #[class = ""]
    Ghost,
}

#[component]
pub async fn timeline(
    cx: &Cx,
    #[default] direction: TimelineDirection,
    #[default] style: TimelineStyle,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            class=(class!(
                "timeline",
                direction.to_class(),
                style.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </ul>
    })
}
