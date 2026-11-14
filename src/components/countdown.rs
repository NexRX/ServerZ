use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

#[allow(dead_code)]
pub const fn clamp_countdown(value: u16) -> u16 {
    if value > 999 { 999 } else { value }
}

#[component]
pub async fn countdown(cx: &Cx, value: u16, #[default] mut attrs: Attributes) -> Result<impl View> {
    let value = clamp_countdown(value);
    Ok(view! {
        <span
            aria-live="polite"
            aria-label=(value.to_string())
            class=(class!("countdown", attrs.remove("class")))
            (attrs)
        >
            <span style=(format!("--value:{value};"))>(value)</span>
        </span>
    })
}

#[cfg(test)]
mod tests {
    use super::clamp_countdown;

    #[test]
    fn countdown_values_are_clamped_to_documented_range() {
        assert_eq!(clamp_countdown(0), 0);
        assert_eq!(clamp_countdown(999), 999);
        assert_eq!(clamp_countdown(1000), 999);
    }
}
