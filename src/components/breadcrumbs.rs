use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn breadcrumbs(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <nav
                aria-label="Breadcrumb"
                class=(class!("breadcrumbs", attrs.remove("class")))
                (attrs)
            >
                <ul>(child)</ul>
            </nav>
        },
    )
}

#[component]
pub async fn breadcrumb_item(
    href: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li class=(class!(attrs.remove("class"))) (attrs)>
            <a href=(href)>(child)</a>
        </li>
    })
}
