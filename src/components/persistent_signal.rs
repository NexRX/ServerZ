//! A runtime [`Signal`] whose value survives full page reloads and is visible to the
//! server.
//!
//! The browser mirrors the signal into a cookie, so the server receives the last
//! value with every request. [`persistent_signal`] reads that cookie when the page
//! renders and uses it as the signal's initial value, which means the first paint is
//! already correct (no flash of the default) and server code can branch on it with
//! [`Signal::get_untracked`] (also available directly on a [`PersistentSignal`]).
//!
//! Runtime links keep a signal alive across navigations on their own; the cookie
//! covers full reloads such as a `POST` followed by a `303 See Other`.
//!
//! The cookie is written by the browser, so the value is **untrusted input**: it is
//! only accepted if it fits the signal's type, otherwise `init` is used. Validate it
//! further before relying on it for anything security-sensitive.
//!
//! ```ignore
//! let menu = persistent_signal(cx, "serverz-app-menu-open", || false);
//!
//! view! {
//!     <div :data-persist=(menu.sync())>
//!         <input type="checkbox" :checked=$(menu.get()) @change=$(|e: Event| menu.set(e.target.checked)) />
//!     </div>
//! }
//! ```
use std::ops::Deref;

use serde::Serialize;
use topcoat::{
    context::Cx,
    router::request::{try_headers, try_original_headers},
    runtime::{Expr, Js, Signal, SignalSurrogate, SignalValue, Surrogate, Surrogated, signal},
};

/// How long the browser keeps the cookie, in seconds (one year).
const MAX_AGE_SECS: u32 = 60 * 60 * 24 * 365;

/// A [`Signal`] mirrored into a cookie under a fixed key.
///
/// A `PersistentSignal` can be captured directly in `$(...)`; use [`sync`](Self::sync)
/// on an element as a bind attribute to activate persistence.
#[derive(Debug)]
pub struct PersistentSignal<T> {
    signal: Signal<T>,
    key: &'static str,
}

impl<T> Clone for PersistentSignal<T> {
    fn clone(&self) -> Self {
        Self { signal: self.signal.clone(), key: self.key }
    }
}

impl<T> Deref for PersistentSignal<T> {
    type Target = Signal<T>;

    fn deref(&self) -> &Self::Target {
        &self.signal
    }
}

/// The runtime representation of [`PersistentSignal`]. It serializes exactly like
/// the underlying signal; `key` is retained only for reconstructing the server-side
/// wrapper when Topcoat evaluates a captured expression.
#[derive(Debug)]
pub struct PersistentSignalSurrogate<T> {
    signal: SignalSurrogate<T>,
    key: &'static str,
}

impl<T> Serialize for PersistentSignalSurrogate<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.signal.serialize(serializer)
    }
}

impl<T> Surrogate for PersistentSignalSurrogate<T> {
    type Real = PersistentSignal<T>;

    fn into_real(self) -> Self::Real {
        PersistentSignal { signal: self.signal.into_real(), key: self.key }
    }
}

impl<T> Surrogated for PersistentSignal<T> {
    type Surrogate = PersistentSignalSurrogate<T>;

    fn into_surrogate(self) -> Self::Surrogate {
        PersistentSignalSurrogate { signal: self.signal.into_surrogate(), key: self.key }
    }
}

impl<T> PersistentSignalSurrogate<T>
where
    T: Surrogated + Clone,
{
    /// Reads the underlying signal in a runtime expression.
    pub fn get(&self) -> T::Surrogate {
        self.signal.get()
    }
}

impl<T> PersistentSignalSurrogate<T>
where
    T: Surrogated,
{
    /// Updates the underlying signal in a runtime expression.
    pub fn set(&self, value: T::Surrogate) {
        self.signal.set(value);
    }
}

/// Creates a [`PersistentSignal`] backed by a cookie named `key`.
///
/// Call [`PersistentSignal::sync`] in a bind attribute to persist changes, and
/// use [`PersistentSignal::signal`] when reading or updating the value in runtime
/// expressions:
///
/// ```ignore
/// let menu = persistent_signal(cx, "app-menu-open", || false);
/// let open = menu.signal();
///
/// view! {
///     <div :data-persist=(menu.sync())>
///         <input :checked=$(open.get()) />
///     </div>
/// }
/// ```
///
/// A valid cookie value is used as the initial value; otherwise `init` is used.
/// The cookie value is untrusted input and should not be used for security
/// decisions without further validation.
///
/// # Panics
///
/// Panics if `key` is empty or contains anything other than ASCII letters, digits,
/// `-` or `_`, since it is used verbatim as a cookie name.
#[track_caller]
pub fn persistent_signal<T>(cx: &Cx, key: &'static str, init: impl FnOnce() -> T) -> PersistentSignal<T>
where
    T: SignalValue,
{
    assert!(
        !key.is_empty() && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
        "persistent signal key `{key}` must only contain ASCII letters, digits, `-` and `_`"
    );

    let stored = read_cookie(cx, key)
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        .and_then(|value| T::from_value(&value));

    PersistentSignal { signal: signal(cx, || stored.unwrap_or_else(init)), key }
}

impl<T> PersistentSignal<T> {
    /// Returns the underlying signal when an API specifically requires a plain
    /// [`Signal`]. Most runtime expressions can capture the persistent signal
    /// directly.
    #[must_use]
    pub fn signal(&self) -> Signal<T> {
        self.signal.clone()
    }

    /// A bind expression that performs the persistence in the browser.
    ///
    /// Attach it to any element as a bind attribute, for example
    /// `:data-persist=(menu.sync())`. It runs on hydration and again whenever the
    /// signal changes, writing the current value to the cookie. It always renders an
    /// empty string.
    #[must_use]
    pub fn sync(&self) -> Expr<String> {
        let signal = self.signal.clone();
        let key = self.key;

        let js = Js::builder()
            .source("(() => { const s = ")
            .surrogate(&signal.clone().into_surrogate())
            .source(format!(
                "; try {{ \
                   document.cookie = '{key}=' \
                     + encodeURIComponent(JSON.stringify(s.dehydrate().v)) \
                     + '; path=/; max-age={MAX_AGE_SECS}; samesite=lax'; \
                 }} catch (_) {{}} \
                 return ''; }})()"
            ))
            .build();

        Expr::evaluate(
            move || {
                // Reading marks the expression dynamic so the browser binds it.
                let _ = signal.read_untracked();
                String::new()
            },
            js,
        )
    }
}

/// The percent-decoded value of the request cookie called `name`, if any.
fn read_cookie(cx: &Cx, name: &str) -> Option<String> {
    let headers = try_original_headers(cx).or_else(|| try_headers(cx))?;
    headers
        .get_all("cookie")
        .iter()
        .filter_map(|header| header.to_str().ok())
        .flat_map(|header| header.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(cookie_name, _)| *cookie_name == name)
        .and_then(|(_, value)| percent_decode(value))
}

fn percent_decode(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = input.get(i + 1..i + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

#[cfg(test)]
mod tests {
    use super::percent_decode;

    #[test]
    fn decodes_json_values() {
        assert_eq!(percent_decode("true").as_deref(), Some("true"));
        assert_eq!(percent_decode("%22a%20b%22").as_deref(), Some("\"a b\""));
    }

    #[test]
    fn rejects_malformed_escapes() {
        assert_eq!(percent_decode("%2"), None);
        assert_eq!(percent_decode("%zz"), None);
    }
}
