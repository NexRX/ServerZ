use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// The wrapper expects the main content followed by eight empty hover zones.
#[component]
pub async fn hover_3d(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("hover-3d", attrs.remove("class"))) (attrs)>
            (child)
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
        </div>
    })
}
