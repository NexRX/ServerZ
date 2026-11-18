use crate::{
    app::{api::auth::me, setup::ensure_setup},
    dayz::Server,
    db::Singleton,
    user::User,
};
use topcoat::{
    context::Cx,
    router::error::{RedirectError, RouterErrorExt as _},
};

/// Returns the current user or falls through to Topcoat's unauthorized response.
pub async fn require_setup(cx: &Cx) -> Result<Server, RedirectError> {
    ensure_setup(cx).await?;
    Server::fetch(cx).await.ok_or_redirect("/setup")
}

/// Returns the current user or falls through to Topcoat's unauthorized response.
pub async fn require_login(cx: &Cx) -> Result<&User, RedirectError> {
    me(cx).await.ok_or_redirect("/login")?.as_ref().ok_or_redirect("/login")
}
