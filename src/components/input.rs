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
