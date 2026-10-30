use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn swap(
    #[default] rotate: bool,
    #[default] flip: bool,
    #[default] active: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let r = if rotate { "swap-rotate" } else { "" };
    let f = if flip { "swap-flip" } else { "" };
    let a = if active { "swap-active" } else { "" };
    Ok(
        view! { <label class=(class!("swap", r, f, a, attrs.remove("class"))) (attrs)><input type="checkbox" checked=(active)>(child)</label> },
    )
}
#[component]
pub async fn swap_on(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <span class=(class!("swap-on", attrs.remove("class"))) (attrs)>(child)</span> })
}
#[component]
pub async fn swap_off(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <span class=(class!("swap-off", attrs.remove("class"))) (attrs)>(child)</span> })
}
#[component]
pub async fn swap_indeterminate(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <span class=(class!("swap-indeterminate", attrs.remove("class"))) (attrs)>(child)</span> },
    )
}
