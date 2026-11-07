use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Tooltip colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TooltipColor {
    #[default]
    #[class = "tooltip-neutral"]
    Neutral,
    #[class = "tooltip-primary"]
    Primary,
    #[class = "tooltip-secondary"]
    Secondary,
    #[class = "tooltip-accent"]
    Accent,
    #[class = "tooltip-info"]
    Info,
    #[class = "tooltip-success"]
    Success,
    #[class = "tooltip-warning"]
    Warning,
    #[class = "tooltip-error"]
    Error,
}

/// Tooltip placements.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TooltipPlacement {
    #[default]
    #[class = "tooltip-top"]
    Top,
    #[class = "tooltip-bottom"]
    Bottom,
    #[class = "tooltip-left"]
    Left,
    #[class = "tooltip-right"]
    Right,
}

/// Tooltip alignments.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TooltipAlignment {
    #[default]
    #[class = "tooltip-center"]
    Center,
    #[class = "tooltip-start"]
    Start,
    #[class = "tooltip-end"]
    End,
}

#[component]
pub async fn tooltip(
    text: String,
    #[default] color: TooltipColor,
    #[default] placement: TooltipPlacement,
    #[default] alignment: TooltipAlignment,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let open_class = if open { "tooltip-open" } else { "" };
    Ok(view! {
        <div
            data-tip=(text)
            class=(class!(
                "tooltip",
                color.to_class(),
                placement.to_class(),
                alignment.to_class(),
                open_class,
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
