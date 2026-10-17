use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn collapse(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <details class=(class!("collapse collapse-arrow", attrs.remove("class"))) (attrs)>(child)</details> },
    )
}

#[component]
pub async fn collapse_typed(
    open: bool,
    plus: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let modifier = if plus {
        "collapse-plus"
    } else {
        "collapse-arrow"
    };
    let state = if open {
        "collapse-open"
    } else {
        "collapse-close"
    };
    Ok(
        view! { <details class=(class!("collapse", modifier, state, attrs.remove("class"))) (attrs) open=(open)>(child)</details> },
    )
}

#[component]
pub async fn collapse_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <summary class=(class!("collapse-title font-semibold", attrs.remove("class"))) (attrs)>(child)</summary> },
    )
}
#[component]
pub async fn collapse_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("collapse-content", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
