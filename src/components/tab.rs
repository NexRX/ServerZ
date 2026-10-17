use crate::components::variants::{ComponentSize, ComponentStyle};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn tabs(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div role="tablist" class=(class!("tabs", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
#[component]
pub async fn tabs_typed(
    style: ComponentStyle,
    size: ComponentSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let s = match style {
        ComponentStyle::Solid => "",
        ComponentStyle::Outline => "tabs-border",
        ComponentStyle::Dash => "",
        ComponentStyle::Soft => "tabs-box",
        ComponentStyle::Ghost => "tabs-lift",
    };
    Ok(
        view! { <div role="tablist" class=(class!("tabs", s, format!("tabs-{}", size.suffix()), attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
#[component]
pub async fn tab(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <button role="tab" class=(class!("tab", attrs.remove("class"))) (attrs)>(child)</button> },
    )
}
#[component]
pub async fn tab_typed(
    active: bool,
    disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let a = if active { "tab-active" } else { "" };
    let d = if disabled { "tab-disabled" } else { "" };
    Ok(
        view! { <button role="tab" aria-selected=(active.then_some("true")) aria-disabled=(disabled.then_some("true")) class=(class!("tab", a, d, attrs.remove("class"))) (attrs)>(child)</button> },
    )
}

#[component]
pub async fn tab_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <section role="tabpanel" class=(class!("tab-content", attrs.remove("class"))) (attrs)>(child)</section> },
    )
}

/// A tab panel with explicit selection and accessibility relationships.
/// Pair `tab_id` with the corresponding tab's `id` and keep `selected` in sync
/// with the radio/button state in the parent tab list.
#[component]
pub async fn tab_panel(
    tab_id: String,
    selected: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let hidden = (!selected).then_some("hidden");
    Ok(
        view! { <section role="tabpanel" aria-labelledby=(tab_id) hidden=(hidden) class=(class!("tab-content", attrs.remove("class"))) (attrs)>(child)</section> },
    )
}

#[component]
pub async fn radio_tab_group(
    name: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div role="tablist" class=(class!("tabs", attrs.remove("class"))) (attrs)><input type="radio" name=(name) class="tab" checked="checked">(child)</div> },
    )
}
