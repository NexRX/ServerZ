use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn hover_gallery(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <figure
                class=(class!("hover-gallery max-w-60", attrs.remove("class")))
                (attrs)
            >
                (child)
            </figure>
        },
    )
}
