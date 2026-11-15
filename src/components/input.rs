use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, expr, signal},
    view::{AttributeValue, Attributes, Child, View, class, component, view},
};

/// Native HTML input types supported by DaisyUI's `.input` component.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Email,
    Number,
    Date,
    DatetimeLocal,
    Week,
    Month,
    Tel,
    Url,
    Search,
    Time,
    Custom(&'static str),
}

impl InputType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Email => "email",
            Self::Number => "number",
            Self::Date => "date",
            Self::DatetimeLocal => "datetime-local",
            Self::Week => "week",
            Self::Month => "month",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Search => "search",
            Self::Time => "time",
            Self::Custom(custom) => custom,
        }
    }
}

/// Input colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum InputColor {
    #[default]
    #[class = "input-neutral"]
    Neutral,
    #[class = "input-primary"]
    Primary,
    #[class = "input-secondary"]
    Secondary,
    #[class = "input-accent"]
    Accent,
    #[class = "input-info"]
    Info,
    #[class = "input-success"]
    Success,
    #[class = "input-warning"]
    Warning,
    #[class = "input-error"]
    Error,
}

/// Input sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum InputSize {
    #[class = "input-xs"]
    Xs,
    #[class = "input-sm"]
    Sm,
    #[default]
    #[class = "input-md"]
    Md,
    #[class = "input-lg"]
    Lg,
    #[class = "input-xl"]
    Xl,
}

/// Input variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum InputVariant {
    #[default]
    #[class = ""]
    Default,
    #[class = "input-ghost"]
    Ghost,
}

/// Reads a string attribute from `attrs`, if present.
fn attr_str(attrs: &Attributes, key: &str) -> Option<String> {
    match attrs.get(key)? {
        AttributeValue::PromotedStr { value, .. } => Some((**value).to_owned()),
        AttributeValue::StaticStr { value, .. } => Some((**value).to_owned()),
        AttributeValue::String { value, .. } => Some(value.clone()),
        _ => None,
    }
}

fn attr_present(attrs: &Attributes, key: &str) -> bool {
    attrs.get(key).is_some_and(AttributeValue::is_present)
}

/// Mirrors the browser check `/^[^\s@]+@[^\s@]+$/`.
fn looks_like_email(value: &str) -> bool {
    let ok = |part: &str| !part.is_empty() && !part.contains(char::is_whitespace);
    value
        .split_once('@')
        .is_some_and(|(local, domain)| ok(local) && ok(domain) && !domain.contains('@'))
}

/// Mirrors the browser check `/^[a-z][a-z0-9+.-]*:\/\/\S+$/i`.
fn looks_like_url(value: &str) -> bool {
    let Some((scheme, rest)) = value.split_once("://") else {
        return false;
    };
    let mut chars = scheme.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
        && !rest.is_empty()
        && !rest.contains(char::is_whitespace)
}

/// A complete DaisyUI input field.
///
/// This is the single entry point for text inputs and supports:
///
/// - native types through `ty`
/// - all DaisyUI colors, sizes, and the ghost variant
/// - optional `legend`, `label`, and `hint` field content
/// - DaisyUI validator styling with `validator: true`
/// - a stable fieldset and input wrapper hierarchy with optional custom child content
///
/// # Automatic validation messages
///
/// The component reads the constraints already present on `inner_attrs`
/// (`required`, `minlength`, `maxlength`, `pattern` + `title`, `min`, `max`)
/// and the input type (`email`, `url`) and renders one message per rule.
/// Every rule is a Topcoat runtime expression bound to a signal that tracks
/// the input's value, so all currently failing rules are shown together and
/// update on each keystroke without a server round trip.
#[component]
pub async fn input(
    cx: &Cx,
    #[default] ty: InputType,
    #[default] color: InputColor,
    #[default] size: InputSize,
    #[default] variant: InputVariant,
    #[default(true)] validator: bool,
    #[default] validation_errors: Vec<String>,
    #[default(true)] rules_tooltip: bool,

    #[default] legend: Option<String>,
    #[default] label: Option<String>,
    #[default] label_for: Option<String>,
    #[default] hint: Option<String>,
    #[default] prefix: Option<String>,
    #[default] suffix: Option<String>,
    #[default] mut attrs: Attributes,
    #[default] mut inner_attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let has_validation_errors = !validation_errors.is_empty();

    // An explicit `ty` wins over a `type` inherited from generated attributes.
    if ty != InputType::Text {
        inner_attrs.remove("type");
    }

    let required = attr_present(&inner_attrs, "required");
    let min_length = attr_str(&inner_attrs, "minlength").and_then(|v| v.parse::<usize>().ok());
    let max_length = attr_str(&inner_attrs, "maxlength").and_then(|v| v.parse::<usize>().ok());
    let min = attr_str(&inner_attrs, "min").and_then(|v| v.parse::<f64>().ok());
    let max = attr_str(&inner_attrs, "max").and_then(|v| v.parse::<f64>().ok());
    let pattern = attr_str(&inner_attrs, "pattern");
    let pattern_message = attr_str(&inner_attrs, "title")
        .unwrap_or_else(|| "Must match the requested format.".to_owned());
    let is_email = ty == InputType::Email;
    let is_url = ty == InputType::Url;
    let initial = attr_str(&inner_attrs, "value").unwrap_or_default();
    let has_rules = required
        || min_length.is_some()
        || max_length.is_some()
        || min.is_some()
        || max.is_some()
        || pattern.is_some()
        || is_email
        || is_url;

    let value = signal(cx, move || initial.clone());
    // Length in UTF-8 bytes, using the runtime's built-in `len` on both sides.
    let count = expr!(value.get().len());
    // `raw!` must return runtime values via `cx.hydrate(..)`, and `${v}` is a
    // runtime string object, so it is converted with `String(..)` first.
    let number = expr!({
        let v = value.get();
        raw!(
            "cx.hydrate(String(${v}).trim() === '' ? NaN : Number(String(${v})))",
            v.trim().parse::<f64>().unwrap_or(f64::NAN)
        )
    });
    let email_ok = expr!({
        let v = value.get();
        raw!(
            "cx.hydrate(/^[^\\s@]+@[^\\s@]+$/.test(String(${v})))",
            looks_like_email(&v)
        )
    });
    let url_ok = expr!({
        let v = value.get();
        raw!(
            "cx.hydrate(/^[a-z][a-z0-9+.-]*:\\/\\/\\S+$/i.test(String(${v})))",
            looks_like_url(&v)
        )
    });
    // The pattern is only evaluated in the browser; the server render treats it as satisfied.
    let pattern_ok = {
        let pattern = pattern.clone().unwrap_or_default();
        expr!({
            let v = value.get();
            let p = pattern.clone();
            raw!(
                "cx.hydrate(String(${v}) === '' || String(${p}) === '' || new RegExp('^(?:' + String(${p}) + ')$', 'v').test(String(${v})))",
                {
                    let _ = (&v, &p);
                    true
                }
            )
        })
    };
    let mut rule_descriptions: Vec<String> = Vec::new();
    if required {
        rule_descriptions.push("Required".to_owned());
    }
    if is_email {
        rule_descriptions.push("Must be a valid email address".to_owned());
    }
    if is_url {
        rule_descriptions.push("Must be a valid URL".to_owned());
    }
    if let Some(n) = min_length {
        rule_descriptions.push(format!("At least {n} characters"));
    }
    if let Some(n) = max_length {
        rule_descriptions.push(format!("At most {n} characters"));
    }
    if let Some(n) = min {
        rule_descriptions.push(format!("Minimum value {n}"));
    }
    if let Some(n) = max {
        rule_descriptions.push(format!("Maximum value {n}"));
    }
    if pattern.is_some() {
        rule_descriptions.push(pattern_message.clone());
    }

    let min_bound = min;
    let max_bound = max;
    let min = min.unwrap_or(f64::NEG_INFINITY);
    let max = max.unwrap_or(f64::INFINITY);
    let min_len = min_length.unwrap_or(0);
    let max_len = max_length.unwrap_or(usize::MAX);

    let oninput = expr!(|e: Event| value.set(e.target.value));

    Ok(view! {
        <fieldset class=(class!("fieldset", attrs.remove("class"))) (attrs)>
            if let Some(legend) = legend {
                <legend class="fieldset-legend">(legend)</legend>
            }
            if let Some(label) = label {
                <label class="label" for=(label_for)>(label)</label>
            }
            <label
                class=(class!(
                    "input",
                    "w-full",
                    color.to_class(),
                    size.to_class(),
                    variant.to_class(),
                    if validator || has_validation_errors { "validator" } else { "" },
                    if has_validation_errors { "input-error" } else { "" },
                ))
            >
                if let Some(prefix) = prefix {
                    (prefix)
                }
                (child)
                <input
                    type=(ty.as_str())
                    class=(class!("grow", inner_attrs.remove("class")))
                    (inner_attrs)
                    @input=(oninput)
                >
                if let Some(suffix) = suffix {
                    (suffix)
                }
                if rules_tooltip && !rule_descriptions.is_empty() {
                    <span
                        class="tooltip tooltip-left cursor-help opacity-60"
                        tabindex="0"
                        aria-label="Validation rules"
                    >
                        <div class="tooltip-content text-left">
                            <ul class="list-disc list-inside whitespace-nowrap">
                                for rule in rule_descriptions {
                                    <li>(rule)</li>
                                }
                            </ul>
                        </div>
                        <svg
                            class="h-[1em]"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2.5"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <circle cx="12" cy="12" r="10"></circle>
                            <path d="M12 16v-4"></path>
                            <path d="M12 8h.01"></path>
                        </svg>
                    </span>
                }
            </label>
            if validator || has_validation_errors || hint.is_some() {
                <div class="validator-hint whitespace-normal">
                    for message in validation_errors {
                        <p>(message)</p>
                    }
                    if has_rules {
                        if required {
                            <p :hidden=$(!value.get().is_empty())>
                                "This field is required."
                            </p>
                        }
                        if is_email {
                            <p :hidden=$(if count > 0 { email_ok } else { true })>
                                "Enter a valid email address."
                            </p>
                        }
                        if is_url {
                            <p :hidden=$(if count > 0 { url_ok } else { true })>
                                "Enter a valid URL."
                            </p>
                        }
                        if let Some(n) = min_length {
                            <p
                                :hidden=$(if count > 0 { count >= min_len } else { true })
                            >
                                "Must be at least "
                                (n)
                                " characters."
                            </p>
                        }
                        if let Some(n) = max_length {
                            <p :hidden=$(count <= max_len)>
                                "Must be at most "
                                (n)
                                " characters."
                            </p>
                        }
                        if pattern.is_some() {
                            <p :hidden=$(pattern_ok)>(pattern_message)</p>
                        }
                        if let Some(n) = min_bound {
                            <p :hidden=$(!(number < min))>
                                "Must be at least "
                                (n)
                                "."
                            </p>
                        }
                        if let Some(n) = max_bound {
                            <p :hidden=$(!(number > max))>
                                "Must be at most "
                                (n)
                                "."
                            </p>
                        }
                    }
                    if let Some(hint) = hint {
                        <p class="opacity-70">(hint)</p>
                    }
                </div>
            }
        </fieldset>
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use topcoat::view::ViewExt;

    fn attrs(pairs: &[(&str, &str)]) -> Attributes {
        let cx = &Cx::new();
        let mut attrs = Attributes::new();
        for (key, value) in pairs {
            attrs.insert(cx, (*key).to_owned(), (*value).to_owned());
        }
        attrs
    }

    async fn render_without_tooltip(inner: Attributes) -> String {
        let cx = &Cx::new();
        view! { input(inner_attrs: inner, rules_tooltip: false) }
            .single()
            .await
            .unwrap()
            .render(cx)
    }

    async fn render(ty: InputType, inner: Attributes, errors: Vec<String>) -> String {
        let cx = &Cx::new();
        view! { input(ty: ty, inner_attrs: inner, validation_errors: errors) }
            .single()
            .await
            .unwrap()
            .render(cx)
    }

    /// Returns the opening `<p ...>` tag that precedes `message`.
    fn tag_for<'a>(html: &'a str, message: &str) -> &'a str {
        let end = html
            .find(message)
            .unwrap_or_else(|| panic!("`{message}` not rendered in {html}"));
        let start = html[..end].rfind("<p").unwrap();
        &html[start..end]
    }

    fn is_hidden(html: &str, message: &str) -> bool {
        tag_for(html, message).contains(" hidden")
    }

    #[test]
    fn email_helper() {
        assert!(looks_like_email("a@b.co"));
        assert!(!looks_like_email("ab.co"));
        assert!(!looks_like_email("@b"));
        assert!(!looks_like_email("a@"));
        assert!(!looks_like_email("a@@b"));
        assert!(!looks_like_email("a b@c"));
    }

    #[test]
    fn url_helper() {
        assert!(looks_like_url("https://example.com"));
        assert!(looks_like_url("ftp+x://host"));
        assert!(!looks_like_url("example.com"));
        assert!(!looks_like_url("://x"));
        assert!(!looks_like_url("1http://x"));
        assert!(!looks_like_url("http://"));
        assert!(!looks_like_url("http://a b"));
    }

    #[test]
    fn attribute_helpers() {
        let a = attrs(&[("minlength", "3")]);
        assert_eq!(attr_str(&a, "minlength").as_deref(), Some("3"));
        assert_eq!(attr_str(&a, "missing"), None);
        assert!(attr_present(&a, "minlength"));
        assert!(!attr_present(&a, "missing"));
    }

    #[tokio::test]
    async fn no_rules_renders_no_messages() {
        let html = render(InputType::Text, Attributes::new(), vec![]).await;
        assert!(!html.contains("This field is required."));
        assert!(!html.contains("data-topcoat-bind:hidden"));
    }

    #[tokio::test]
    async fn required_rule() {
        let msg = "This field is required.";
        let empty = render(InputType::Text, attrs(&[("required", "")]), vec![]).await;
        assert!(!is_hidden(&empty, msg));
        assert!(empty.contains("data-topcoat-bind:hidden"));

        let filled = render(
            InputType::Text,
            attrs(&[("required", ""), ("value", "x")]),
            vec![],
        )
        .await;
        assert!(is_hidden(&filled, msg));
    }

    #[tokio::test]
    async fn minlength_rule() {
        let msg = "Must be at least 3 characters.";
        let short = attrs(&[("minlength", "3"), ("value", "ab")]);
        assert!(!is_hidden(
            &render(InputType::Text, short, vec![]).await,
            msg
        ));
        let ok = attrs(&[("minlength", "3"), ("value", "abc")]);
        assert!(is_hidden(&render(InputType::Text, ok, vec![]).await, msg));
        let empty = attrs(&[("minlength", "3")]);
        assert!(is_hidden(
            &render(InputType::Text, empty, vec![]).await,
            msg
        ));
    }

    #[tokio::test]
    async fn maxlength_rule() {
        let msg = "Must be at most 3 characters.";
        let long = attrs(&[("maxlength", "3"), ("value", "abcd")]);
        assert!(!is_hidden(
            &render(InputType::Text, long, vec![]).await,
            msg
        ));
        let ok = attrs(&[("maxlength", "3"), ("value", "abc")]);
        assert!(is_hidden(&render(InputType::Text, ok, vec![]).await, msg));
    }

    #[tokio::test]
    async fn length_is_measured_in_utf8_bytes() {
        let msg = "Must be at most 2 characters.";
        let a = attrs(&[("maxlength", "2"), ("value", "éé")]);
        assert!(!is_hidden(&render(InputType::Text, a, vec![]).await, msg));
    }

    #[tokio::test]
    async fn min_and_max_rules() {
        let min_msg = "Must be at least 1.";
        let max_msg = "Must be at most 10.";
        let low = attrs(&[("min", "1"), ("max", "10"), ("value", "0")]);
        let html = render(InputType::Number, low, vec![]).await;
        assert!(!is_hidden(&html, min_msg));
        assert!(is_hidden(&html, max_msg));

        let high = attrs(&[("min", "1"), ("max", "10"), ("value", "11")]);
        let html = render(InputType::Number, high, vec![]).await;
        assert!(is_hidden(&html, min_msg));
        assert!(!is_hidden(&html, max_msg));

        let ok = attrs(&[("min", "1"), ("max", "10"), ("value", "5")]);
        let html = render(InputType::Number, ok, vec![]).await;
        assert!(is_hidden(&html, min_msg));
        assert!(is_hidden(&html, max_msg));

        let empty = attrs(&[("min", "1"), ("max", "10")]);
        let html = render(InputType::Number, empty, vec![]).await;
        assert!(is_hidden(&html, min_msg));
        assert!(is_hidden(&html, max_msg));
    }

    #[tokio::test]
    async fn email_rule() {
        let msg = "Enter a valid email address.";
        let bad = attrs(&[("value", "nope")]);
        assert!(!is_hidden(
            &render(InputType::Email, bad, vec![]).await,
            msg
        ));
        let ok = attrs(&[("value", "a@b.co")]);
        assert!(is_hidden(&render(InputType::Email, ok, vec![]).await, msg));
        let empty = Attributes::new();
        assert!(is_hidden(
            &render(InputType::Email, empty, vec![]).await,
            msg
        ));
    }

    #[tokio::test]
    async fn url_rule() {
        let msg = "Enter a valid URL.";
        let bad = attrs(&[("value", "example.com")]);
        assert!(!is_hidden(&render(InputType::Url, bad, vec![]).await, msg));
        let ok = attrs(&[("value", "https://example.com")]);
        assert!(is_hidden(&render(InputType::Url, ok, vec![]).await, msg));
    }

    #[tokio::test]
    async fn pattern_rule_uses_title_and_is_browser_bound() {
        let a = attrs(&[("pattern", "[a-z]+"), ("title", "Letters only")]);
        let html = render(InputType::Text, a, vec![]).await;
        assert!(html.contains("Letters only"));
        assert!(html.contains("data-topcoat-bind:hidden"));
        assert!(html.contains("new RegExp"));

        let default = attrs(&[("pattern", "[a-z]+")]);
        let html = render(InputType::Text, default, vec![]).await;
        assert!(html.contains("Must match the requested format."));
    }

    #[tokio::test]
    async fn multiple_rules_can_fail_together() {
        let a = attrs(&[
            ("required", ""),
            ("minlength", "5"),
            ("pattern", "[0-9]+"),
            ("value", "ab"),
        ]);
        let html = render(InputType::Text, a, vec![]).await;
        assert!(is_hidden(&html, "This field is required."));
        assert!(!is_hidden(&html, "Must be at least 5 characters."));
    }

    #[tokio::test]
    async fn server_errors_render_without_rules() {
        let html = render(
            InputType::Text,
            Attributes::new(),
            vec!["Username taken".into()],
        )
        .await;
        assert!(html.contains("Username taken"));
        assert!(html.contains("input-error"));
        assert!(html.contains("validator"));
    }

    /// Every `raw!` snippet must return a runtime value, not a JS primitive,
    /// or the browser binding throws and the message never updates.
    #[tokio::test]
    async fn raw_javascript_returns_runtime_values() {
        let html = render(InputType::Email, attrs(&[("min", "1")]), vec![]).await;
        assert!(html.contains("cx.hydrate(/"), "{html}");
        let html = render(InputType::Number, attrs(&[("min", "1")]), vec![]).await;
        assert!(html.contains("cx.hydrate(String("), "{html}");
        let html = render(InputType::Text, attrs(&[("pattern", "a")]), vec![]).await;
        assert!(html.contains("cx.hydrate(String("), "{html}");
        assert!(!html.contains("[..."), "{html}");
    }

    #[tokio::test]
    async fn tooltip_lists_all_rules() {
        let a = attrs(&[
            ("required", ""),
            ("minlength", "3"),
            ("maxlength", "30"),
            ("pattern", "[a-z]+"),
            ("title", "Letters only"),
        ]);
        let html = render(InputType::Text, a, vec![]).await;
        let tip = &html[html.find("tooltip-content").expect("tooltip")..];
        for rule in [
            "Required",
            "At least 3 characters",
            "At most 30 characters",
            "Letters only",
        ] {
            assert!(tip.contains(rule), "{rule} missing in {html}");
        }
    }

    #[tokio::test]
    async fn tooltip_includes_type_and_range_rules() {
        let html = render(InputType::Email, attrs(&[("min", "1")]), vec![]).await;
        assert!(html.contains("Must be a valid email address"), "{html}");
        assert!(html.contains("Minimum value 1"), "{html}");
        let html = render(InputType::Url, Attributes::new(), vec![]).await;
        assert!(html.contains("Must be a valid URL"), "{html}");
    }

    #[tokio::test]
    async fn tooltip_can_be_disabled() {
        let html = render_without_tooltip(attrs(&[("required", "")])).await;
        assert!(!html.contains("tooltip"), "{html}");
        assert!(html.contains("This field is required."), "{html}");
    }

    #[tokio::test]
    async fn no_tooltip_without_rules() {
        let html = render(InputType::Text, Attributes::new(), vec![]).await;
        assert!(!html.contains("tooltip"), "{html}");
    }

    #[tokio::test]
    async fn input_wires_value_tracking() {
        let html = render(InputType::Text, attrs(&[("required", "")]), vec![]).await;
        assert!(html.contains("data-topcoat-on:input"));
    }
}
