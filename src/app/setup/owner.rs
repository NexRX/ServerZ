use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

#[page]
pub async fn setup_owner(cx: &Cx) -> Result<impl View> {
    Ok(view! { <main>"Setup Owner"</main> })
}
