use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

#[page]
pub async fn setup_owner() -> Result<impl View> {
    Ok(view! { <main>"Setup Owner"</main> })
}
