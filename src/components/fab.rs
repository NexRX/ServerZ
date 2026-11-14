use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn fab(
    cx: &Cx,
    #[default] flower: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let layout = if flower { "fab-flower" } else { "" };
    Ok(view! {
        <div class=(class!("fab", layout, attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn fab_action(
    cx: &Cx,
    #[default] label: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(attrs.remove("class"))) (attrs)>
            (label)
            <button class="btn btn-circle">(child)</button>
        </div>
    })
}

#[component]
pub async fn fab_tooltip(
    cx: &Cx,
    #[default] label: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!("tooltip tooltip-left", attrs.remove("class")))
            data-tip=(label)
            (attrs)
        >
            (child)
        </div>
    })
}

#[component]
pub async fn fab_main_action(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("fab-main-action", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}
#[component]
pub async fn fab_close(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("fab-close", attrs.remove("class"))) (attrs)>(child)</div>
    })
}
