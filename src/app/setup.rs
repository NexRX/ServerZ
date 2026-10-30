use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

#[page]
pub async fn setup() -> Result<impl View> {
    Ok(view! {
        <main>
            "setup"
        </main>
    })
}
