use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Steps directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum StepsDirection {
    #[default]
    #[class = "steps-horizontal"]
    Horizontal,
    #[class = "steps-vertical"]
    Vertical,
}

/// Steps colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum StepColor {
    #[default]
    #[class = "step-neutral"]
    Neutral,
    #[class = "step-primary"]
    Primary,
    #[class = "step-secondary"]
    Secondary,
    #[class = "step-accent"]
    Accent,
    #[class = "step-info"]
    Info,
    #[class = "step-success"]
    Success,
    #[class = "step-warning"]
    Warning,
    #[class = "step-error"]
    Error,
}

#[component]
pub async fn steps(
    #[default] direction: StepsDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            class=(class!("steps", direction.to_class(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </ul>
    })
}

#[component]
pub async fn step(
    color: StepColor,
    content: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li
            data-content=(content)
            class=(class!("step", color.to_class(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </li>
    })
}
