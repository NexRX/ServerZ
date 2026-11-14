#![allow(dead_code, non_upper_case_globals)]

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, Class, StaticClass, View, class, component, view},
};

/// daisyUI button colors. Defaults to `Primary`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonColor {
    #[default]
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
    Neutral,
}

impl ButtonColor {
    fn classes(self) -> StaticClass {
        match self {
            Self::Primary => class!("btn btn-primary"),
            Self::Secondary => class!("btn btn-secondary"),
            Self::Accent => class!("btn btn-accent"),
            Self::Info => class!("btn btn-info"),
            Self::Success => class!("btn btn-success"),
            Self::Warning => class!("btn btn-warning"),
            Self::Error => class!("btn btn-error"),
            Self::Neutral => class!("btn btn-neutral"),
        }
    }
}

/// daisyUI button styles. Defaults to a filled button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Solid,
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
}

impl ButtonVariant {
    /// Backwards-compatible alias for the former primary variant.
    pub const Primary: Self = Self::Solid;
    /// Backwards-compatible alias for the former destructive variant.
    pub const Destructive: Self = Self::Soft;

    fn classes(self) -> StaticClass {
        match self {
            Self::Solid => class!("btn"),
            Self::Outline => class!("btn-outline"),
            Self::Dash => class!("btn-dash"),
            Self::Soft => class!("btn-soft"),
            Self::Ghost => class!("btn-ghost"),
            Self::Link => class!("btn-link"),
        }
    }
}

/// daisyUI button sizes. Defaults to `Md`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ButtonSize {
    fn classes(self) -> StaticClass {
        match self {
            Self::Xs => class!("btn-xs"),
            Self::Sm => class!("btn-sm"),
            Self::Md => class!("btn-md"),
            Self::Lg => class!("btn-lg"),
            Self::Xl => class!("btn-xl"),
        }
    }
}

/// daisyUI button layout modifiers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonLayout {
    #[default]
    Normal,
    Wide,
    Block,
    Square,
    Circle,
}

impl ButtonLayout {
    fn classes(self) -> StaticClass {
        match self {
            Self::Normal => class!("btn"),
            Self::Wide => class!("btn-wide"),
            Self::Block => class!("btn-block"),
            Self::Square => class!("btn-square"),
            Self::Circle => class!("btn-circle"),
        }
    }
}

/// Builds daisyUI button classes for a non-button element such as a link.
#[must_use]
pub fn button_variants(
    color: ButtonColor,
    variant: ButtonVariant,
    size: ButtonSize,
    layout: ButtonLayout,
) -> Class<(StaticClass, StaticClass, StaticClass)> {
    let _ = layout;
    class!(color.classes(), variant.classes(), size.classes())
}

/// A daisyUI styled button. Attributes and children are forwarded normally.
#[component]
pub async fn button(
    cx: &Cx,
    #[default] color: ButtonColor,
    #[default] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] layout: ButtonLayout,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            class=(class!(
                button_variants(color, variant, size, layout),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </button>
    })
}
