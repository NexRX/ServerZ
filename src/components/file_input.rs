use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn file_input(cx: &Cx, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <input type="file" class=(class!("file-input", attrs.remove("class"))) (attrs)>
    })
}
