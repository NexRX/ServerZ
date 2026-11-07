use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn mockup_code(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("mockup-code", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn mockup_code_line(
    #[default] prefix: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <pre data-prefix=(prefix) class=(class!(attrs.remove("class"))) (attrs)>
                <code>(child)</code>
            </pre>
        },
    )
}
