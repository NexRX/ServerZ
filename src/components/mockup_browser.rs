use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn mockup_browser(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("mockup-browser", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

#[component]
pub async fn mockup_browser_toolbar(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!("mockup-browser-toolbar", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
