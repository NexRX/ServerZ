use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
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

#[component]
async fn input_control(
    cx: &Cx,
    ty: InputType,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <input type=(ty.as_str()) class=(class!("grow", attrs.remove("class"))) (attrs)>
    })
}

/// A complete DaisyUI input field.
///
/// This is the single entry point for text inputs and supports:
///
/// - native types through `input_type`
/// - all DaisyUI colors, sizes, and the ghost variant
/// - optional `legend`, `label`, and `hint` field content
/// - native browser validation attributes forwarded through `attrs`
/// - DaisyUI validator styling with `validator: true`
/// - a stable fieldset and input wrapper hierarchy with optional custom child content
///
/// The `id`, `name`, `placeholder`, `required`, `pattern`, `min`, `max`,
/// `minlength`, `maxlength`, `list`, `disabled`, and accessibility attributes
/// are all passed directly to the native `<input>` element.
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
    #[default] inner_attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let has_validation_errors = !validation_errors.is_empty();
    let validation_message = if has_validation_errors {
        Some(validation_errors.join("\n"))
    } else {
        hint
    };

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
                input_control(ty: ty, attrs: inner_attrs)
                if let Some(suffix) = suffix {
                    (suffix)
                }
            </label>
            if let Some(message) = validation_message {
                <p class="validator-hint whitespace-pre-line">(message)</p>
            }
        </fieldset>
    })
}
