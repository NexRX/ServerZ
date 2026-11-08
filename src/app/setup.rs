mod owner;
mod server;

use topcoat::{
    Result,
    context::Cx,
    router::{error::RouterErrorExt, page},
    view::{View, view},
};

use crate::{server::ensure_server_created, user::ensure_owner_created};

#[page]
pub async fn setup(cx: &Cx) -> Result<impl View> {
    ensure_owner_created(cx)
        .await
        .ok_or_redirect("/setup/owner")?;
    ensure_server_created(cx)
        .await
        .ok_or_redirect("/setup/server")?;

    Ok(view! { <main>"Setup Complete"</main> })
}
