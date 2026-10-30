use crate::{db::Singleton, server::Server};
use topcoat::{
    context::Cx,
    router::error::{RedirectError, RouterErrorExt as _},
};

/// Returns the current user or falls through to Topcoat's unauthorized response.
pub async fn require_server(cx: &Cx) -> Result<Server, RedirectError> {
    Server::fetch(cx).await.ok_or_redirect("/setup")
}
