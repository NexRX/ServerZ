use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Menu sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum MenuSize {
    #[class = "menu-xs"]
    Xs,
    #[class = "menu-sm"]
    Sm,
    #[default]
    #[class = "menu-md"]
    Md,
    #[class = "menu-lg"]
    Lg,
    #[class = "menu-xl"]
    Xl,
}

/// Menu directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum MenuDirection {
    #[default]
    #[class = "menu-horizontal"]
    Horizontal,
    #[class = "menu-vertical"]
    Vertical,
}

#[component]
pub async fn menu(
    #[default] direction: MenuDirection,
    #[default] size: MenuSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            class=(class!(
                "menu",
                direction.to_class(),
                size.to_class(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </ul>
    })
}
