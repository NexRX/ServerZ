use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Carousel directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum CarouselDirection {
    #[default]
    #[class = "carousel-horizontal"]
    Horizontal,
    #[class = "carousel-vertical"]
    Vertical,
}

/// Carousel alignments.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum CarouselAlignment {
    #[default]
    #[class = "carousel-start"]
    Start,
    #[class = "carousel-center"]
    Center,
    #[class = "carousel-end"]
    End,
}

#[component]
pub async fn carousel(
    cx: &Cx,
    #[default] direction: CarouselDirection,
    #[default] alignment: CarouselAlignment,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(
                "carousel",
                direction.to_class(),
                alignment.to_class(),
                "scroll-smooth",
                attrs.remove("class"),
            ))
            (attrs)
            tabindex="0"
        >
            (child)
        </div>
    })
}

#[component]
pub async fn carousel_item(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("carousel-item", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}
