use crate::components::variants::{AuraSize, AuraStyle};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn aura(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("aura aura-glow", attrs.remove("class"))) (attrs)>(child)</div> })
}

#[component]
pub async fn aura_typed(
    style: AuraStyle,
    size: AuraSize,
    duration_ms: u32,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("aura", format!("aura-{}", style.suffix()), format!("aura-{}", size.suffix()), format!("duration-{duration_ms}"), attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
