use anyhow::Result;
use topcoat::{
    context::Cx,
    router::error::{RedirectError, redirect},
};
use tracing::{error, instrument};

pub trait Guard: Sized {
    const GUARD_NAME: &str;
    const REDIRECT_AVAILABLE: &str = "/";
    const REDIRECT_UNAVAILABLE: &str;

    async fn fetch_available(cx: &Cx) -> Result<Self>;

    async fn is_available(cx: &Cx) -> Result<bool> {
        Ok(Self::fetch_available(cx).await.is_ok())
    }

    async fn is_available_or_default(cx: &Cx) -> bool {
        Self::is_available(cx).await.unwrap_or_else(|err| {
            error!(?err, "Availability check failed");
            false
        })
    }

    #[allow(dead_code)]
    #[instrument(level = "debug", skip_all, fields(guard = %Self::GUARD_NAME), err)]
    async fn require(cx: &Cx) -> Result<Self, RedirectError> {
        match Self::fetch_available(cx).await {
            Ok(v) => Ok(v),
            Err(err) => {
                error!(?err, "Required but unavailable");
                Err(redirect(Self::REDIRECT_UNAVAILABLE))
            }
        }
    }

    #[instrument(level = "debug", skip_all, fields(guard = %Self::GUARD_NAME), ret, err)]
    async fn ensure_available(cx: &Cx) -> Result<(), RedirectError> {
        match Self::is_available_or_default(cx).await {
            true => Ok(()),
            false => Err(redirect(Self::REDIRECT_UNAVAILABLE)),
        }
    }

    #[instrument(level = "debug", skip_all, fields(guard = %Self::GUARD_NAME), ret, err)]
    async fn ensure_unavailable(cx: &Cx) -> Result<(), RedirectError> {
        match Self::is_available_or_default(cx).await {
            true => Err(redirect(Self::REDIRECT_AVAILABLE)),
            false => Ok(()),
        }
    }
}
