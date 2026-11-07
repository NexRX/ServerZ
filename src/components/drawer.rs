use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn drawer(
    #[default] end: bool,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let placement = end.then_some("drawer-end");
    let state = open.then_some("drawer-open");
    Ok(view! {
        <div class=(class!("drawer", placement, state, attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

#[component]
pub async fn drawer_toggle(id: String, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! {
            <input
                id=(id)
                type="checkbox"
                autocomplete="off"
                checked=(attrs.remove("checked").is_some())
                class=(class!("drawer-toggle", attrs.remove("class")))
                (attrs)
            >
        },
    )
}
#[component]
pub async fn drawer_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("drawer-content", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}
#[component]
pub async fn drawer_side(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("drawer-side", attrs.remove("class"))) (attrs)>(child)</div>
    })
}
#[component]
pub async fn drawer_overlay(for_id: String, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! {
            <label
                for=(for_id)
                aria-label="close sidebar"
                class=(class!("drawer-overlay", attrs.remove("class")))
                (attrs)
            ></label>
        },
    )
}
#[component]
pub async fn drawer_button(
    for_id: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <label
                for=(for_id)
                class=(class!("drawer-button", attrs.remove("class")))
                (attrs)
            >
                (child)
            </label>
        },
    )
}
