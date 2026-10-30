use crate::components::variants::{ComponentDirection, ComponentSize};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn menu(
    #[default] direction: ComponentDirection,
    #[default] size: ComponentSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <ul class=(class!("menu", format!("menu-{}", direction.suffix()), format!("menu-{}", size.suffix()), attrs.remove("class"))) (attrs)>(child)</ul> },
    )
}
#[component]
pub async fn menu_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <li class=(class!("menu-title", attrs.remove("class"))) (attrs)>(child)</li> })
}
#[component]
pub async fn menu_item(
    active: bool,
    disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let a = if active { "menu-active" } else { "" };
    let d = if disabled { "menu-disabled" } else { "" };
    Ok(view! { <li class=(class!(a, d, attrs.remove("class"))) (attrs)>(child)</li> })
}

#[component]
pub async fn menu_dropdown(
    title: String,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <li class=(class!(attrs.remove("class"))) (attrs)><details open=(open)><summary>(title)</summary><ul>(child)</ul></details></li> },
    )
}

#[component]
pub async fn menu_paged(
    title: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <li class=(class!("menu-paged", attrs.remove("class"))) (attrs)><details><summary>(title)</summary><ul>(child)</ul></details></li> },
    )
}

#[component]
pub async fn menu_dropdown_toggle(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <button class=(class!("menu-dropdown-toggle", attrs.remove("class"))) (attrs)>(child)</button> },
    )
}
