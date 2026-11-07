use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn toast(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <div role="status" class=(class!("toast", attrs.remove("class"))) (attrs)>
                (child)
            </div>
        },
    )
}
