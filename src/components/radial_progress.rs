use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// Radial sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadialSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl RadialSize {
    /// Returns the CSS value string for this size.
    pub const fn to_css(self) -> &'static str {
        match self {
            Self::Sm => "4rem",
            Self::Md => "5rem",
            Self::Lg => "7rem",
        }
    }
}

/// Radial thicknesses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadialThickness {
    Thin,
    #[default]
    Normal,
    Thick,
}

impl RadialThickness {
    /// Returns the CSS value string for this thickness.
    pub const fn to_css(self) -> &'static str {
        match self {
            Self::Thin => "4px",
            Self::Normal => "8px",
            Self::Thick => "12px",
        }
    }
}

#[component]
pub async fn radial_progress(
    #[default] value: u8,
    #[default] size: RadialSize,
    #[default] thickness: RadialThickness,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let value = value.min(100);
    Ok(
        view! {
            <div
                role="progressbar"
                aria-valuenow=(value.to_string())
                style=(format!(
                    "--value:{value};--size:{};--thickness:{};",
                    size.to_css(),
                    thickness.to_css(),
                ))
                class=(class!("radial-progress", attrs.remove("class")))
                (attrs)
            >
                (format!("{value}%"))
            </div>
        },
    )
}
