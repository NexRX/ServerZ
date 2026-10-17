#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComponentColor {
    #[default]
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl ComponentColor {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Accent => "accent",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComponentSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ComponentSize {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComponentStyle {
    #[default]
    Solid,
    Outline,
    Dash,
    Soft,
    Ghost,
}
impl ComponentStyle {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Solid => "",
            Self::Outline => "outline",
            Self::Dash => "dash",
            Self::Soft => "soft",
            Self::Ghost => "ghost",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComponentDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselAlignment {
    #[default]
    Start,
    Center,
    End,
}
impl CarouselAlignment {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ModalPlacement {
    #[default]
    Middle,
    Top,
    Bottom,
    Start,
    End,
}
impl ModalPlacement {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Middle => "middle",
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Start => "start",
            Self::End => "end",
        }
    }
}
impl ComponentDirection {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipAlignment {
    #[default]
    Center,
    Start,
    End,
}
impl TooltipAlignment {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Center => "center",
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RatingMode {
    #[default]
    Full,
    Half,
    ReadOnly,
}
impl RatingMode {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Full => "",
            Self::Half => "rating-half",
            Self::ReadOnly => "",
        }
    }
}
impl TooltipPlacement {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LoadingStyle {
    #[default]
    Spinner,
    Dots,
    Ring,
    Ball,
    Bars,
    Infinity,
}
impl LoadingStyle {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Spinner => "spinner",
            Self::Dots => "dots",
            Self::Ring => "ring",
            Self::Ball => "ball",
            Self::Bars => "bars",
            Self::Infinity => "infinity",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComponentVariant {
    #[default]
    Default,
    Ghost,
}
impl ComponentVariant {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Ghost => "ghost",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadialSize {
    Sm,
    #[default]
    Md,
    Lg,
}
impl RadialSize {
    pub const fn css(self) -> &'static str {
        match self {
            Self::Sm => "4rem",
            Self::Md => "5rem",
            Self::Lg => "7rem",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadialThickness {
    Thin,
    #[default]
    Normal,
    Thick,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaskShape {
    #[default]
    Squircle,
    Heart,
    Hexagon,
    Circle,
    Star,
    Triangle,
    Diamond,
}
impl MaskShape {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Squircle => "squircle",
            Self::Heart => "heart",
            Self::Hexagon => "hexagon",
            Self::Circle => "circle",
            Self::Star => "star",
            Self::Triangle => "triangle",
            Self::Diamond => "diamond",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaskHalf {
    #[default]
    None,
    First,
    Second,
}
impl MaskHalf {
    pub const fn class(self) -> &'static str {
        match self {
            Self::None => "",
            Self::First => "mask-half-1",
            Self::Second => "mask-half-2",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MegamenuLayout {
    #[default]
    Normal,
    Wide,
    Full,
    Vertical,
}
impl MegamenuLayout {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Normal => "",
            Self::Wide => "megamenu-wide",
            Self::Full => "megamenu-full",
            Self::Vertical => "megamenu-vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AuraSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}
impl AuraSize {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AuraStyle {
    #[default]
    Glow,
    Rainbow,
    Dual,
    Holo,
    Gold,
    Silver,
}
impl AuraStyle {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Glow => "glow",
            Self::Rainbow => "rainbow",
            Self::Dual => "dual",
            Self::Holo => "holo",
            Self::Gold => "gold",
            Self::Silver => "silver",
        }
    }
}
impl RadialThickness {
    pub const fn css(self) -> &'static str {
        match self {
            Self::Thin => "4px",
            Self::Normal => "8px",
            Self::Thick => "12px",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_color_and_size_mappings_are_stable() {
        assert_eq!(ComponentColor::Primary.suffix(), "primary");
        assert_eq!(ComponentColor::Error.suffix(), "error");
        assert_eq!(ComponentSize::Xs.suffix(), "xs");
        assert_eq!(ComponentSize::Xl.suffix(), "xl");
    }

    #[test]
    fn component_style_and_direction_mappings_are_stable() {
        assert_eq!(ComponentStyle::Outline.suffix(), "outline");
        assert_eq!(ComponentStyle::Ghost.suffix(), "ghost");
        assert_eq!(ComponentDirection::Horizontal.suffix(), "horizontal");
        assert_eq!(ComponentDirection::Vertical.suffix(), "vertical");
    }

    #[test]
    fn specialized_class_mappings_are_stable() {
        assert_eq!(MaskHalf::First.class(), "mask-half-1");
        assert_eq!(MegamenuLayout::Full.class(), "megamenu-full");
        assert_eq!(AuraStyle::Rainbow.suffix(), "rainbow");
        assert_eq!(RadialThickness::Thick.css(), "12px");
    }
}
