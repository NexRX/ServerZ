use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

#[page]
pub async fn setup_server() -> Result<impl View> {
    Ok(view! { <main>"Setup Server"</main> })
}
