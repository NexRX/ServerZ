use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn dropdown(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <details class=(class!("dropdown", attrs.remove("class"))) (attrs)>
                (child)
            </details>
        },
    )
}

#[component]
pub async fn dropdown_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <ul
                class=(class!(
                    "dropdown-content menu z-10 w-52 rounded-box bg-base-100 p-2 shadow",
                    attrs.remove("class"),
                ))
                (attrs)
            >
                (child)
            </ul>
        },
    )
}
