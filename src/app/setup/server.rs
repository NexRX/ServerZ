use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

#[page]
pub async fn setup_server(cx: &Cx) -> Result<impl View> {
    Ok(view! { <main>"Setup Server"</main> })
}
