use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Aura styles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum AuraStyle {
    #[default]
    #[class = "aura-glow"]
    Glow,
    #[class = "aura-rainbow"]
    Rainbow,
    #[class = "aura-dual"]
    Dual,
    #[class = "aura-holo"]
    Holo,
    #[class = "aura-gold"]
    Gold,
    #[class = "aura-silver"]
    Silver,
}

/// Aura sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum AuraSize {
    #[class = "aura-xs"]
    Xs,
    #[class = "aura-sm"]
    Sm,
    #[default]
    #[class = "aura-md"]
    Md,
    #[class = "aura-lg"]
    Lg,
    #[class = "aura-xl"]
    Xl,
}

#[component]
pub async fn aura(
    cx: &Cx,
    #[default] style: AuraStyle,
    #[default] size: AuraSize,
    #[default] duration_ms: u32,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(
                "aura",
                style.to_class(),
                size.to_class(),
                format!("duration-{duration_ms}"),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
