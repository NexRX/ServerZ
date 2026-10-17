#![allow(dead_code)]

use crate::components::variants::{ComponentColor, ComponentSize};
use topcoat::{
    Result,
    view::{Attributes, StaticClass, View, class, component, view},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputColor {
    #[default]
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}
impl InputColor {
    fn classes(self) -> StaticClass {
        match self {
            Self::Neutral => class!("input"),
            Self::Primary => class!("input-primary"),
            Self::Secondary => class!("input-secondary"),
            Self::Accent => class!("input-accent"),
            Self::Info => class!("input-info"),
            Self::Success => class!("input-success"),
            Self::Warning => class!("input-warning"),
            Self::Error => class!("input-error"),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}
impl InputSize {
    fn classes(self) -> StaticClass {
        match self {
            Self::Xs => class!("input-xs"),
            Self::Sm => class!("input-sm"),
            Self::Md => class!("input-md"),
            Self::Lg => class!("input-lg"),
            Self::Xl => class!("input-xl"),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputVariant {
    #[default]
    Default,
    Ghost,
}
impl InputVariant {
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("input"),
            Self::Ghost => class!("input-ghost"),
        }
    }
}

#[component]
pub async fn input(
    #[default] color: InputColor,
    #[default] size: InputSize,
    #[default] variant: InputVariant,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <input class=(class!("input w-full", color.classes(), size.classes(), variant.classes(), attrs.remove("class"))) (attrs)> },
    )
}

#[component]
pub async fn input_typed(
    color: ComponentColor,
    size: ComponentSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <input class=(class!("input w-full", format!("input-{}", color.suffix()), format!("input-{}", size.suffix()), attrs.remove("class"))) (attrs)> },
    )
}
