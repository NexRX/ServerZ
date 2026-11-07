use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn pagination(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <nav
                aria-label="Pagination"
                class=(class!("join", attrs.remove("class")))
                (attrs)
            >
                (child)
            </nav>
        },
    )
}

#[component]
pub async fn pagination_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <button class=(class!("btn join-item", attrs.remove("class"))) (attrs)>
                (child)
            </button>
        },
    )
}
