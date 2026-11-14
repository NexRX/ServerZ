#![allow(dead_code)]

use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// daisyUI card variants. Defaults to the standard card.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    #[default]
    Default,
    Border,
    Dash,
    Side,
}

impl CardVariant {
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("card"),
            Self::Border => class!("card-border"),
            Self::Dash => class!("card-dash"),
            Self::Side => class!("card-side"),
        }
    }
}

/// daisyUI card sizes. Defaults to `Md`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl CardSize {
    fn classes(self) -> StaticClass {
        match self {
            Self::Sm => class!("card-sm"),
            Self::Md => class!("card-md"),
            Self::Lg => class!("card-lg"),
            Self::Xl => class!("card-xl"),
        }
    }
}

/// daisyUI card layout modifiers. Defaults to a normal-width card.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardLayout {
    #[default]
    Normal,
    Compact,
    ImageFull,
}

impl CardLayout {
    fn classes(self) -> StaticClass {
        match self {
            Self::Normal => class!("card"),
            Self::Compact => class!("card-compact"),
            Self::ImageFull => class!("image-full"),
        }
    }
}

#[component]
pub async fn card(
    cx: &Cx,
    #[default] variant: CardVariant,
    #[default] size: CardSize,
    #[default] layout: CardLayout,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(
                "card",
                variant.classes(),
                size.classes(),
                layout.classes(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

#[component]
pub async fn card_header(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("card-title", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn card_title(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <h2 class=(class!(attrs.remove("class"))) (attrs)>(child)</h2> })
}

#[component]
pub async fn card_description(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!("text-base-content/70", attrs.remove("class"))) (attrs)>
            (child)
        </p>
    })
}

#[component]
pub async fn card_content(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("card-body", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn card_footer(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("card-actions", attrs.remove("class"))) (attrs)>(child)</div>
    })
}
