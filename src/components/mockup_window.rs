use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn mockup_window(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("mockup-window", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}
