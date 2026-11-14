use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

/// Loading styles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum LoadingStyle {
    #[default]
    #[class = "loading-spinner"]
    Spinner,
    #[class = "loading-dots"]
    Dots,
    #[class = "loading-ring"]
    Ring,
    #[class = "loading-ball"]
    Ball,
    #[class = "loading-bars"]
    Bars,
    #[class = "loading-infinity"]
    Infinity,
}

/// Loading sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum LoadingSize {
    #[class = "loading-xs"]
    Xs,
    #[class = "loading-sm"]
    Sm,
    #[default]
    #[class = "loading-md"]
    Md,
    #[class = "loading-lg"]
    Lg,
    #[class = "loading-xl"]
    Xl,
}

#[component]
pub async fn loading(
    cx: &Cx,
    #[default] style: LoadingStyle,
    #[default] size: LoadingSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            role="status"
            aria-label="Loading"
            class=(class!(
                "loading",
                style.to_class(),
                size.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        ></span>
    })
}
