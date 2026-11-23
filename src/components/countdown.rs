use topcoat::{
    Result,
    context::Cx,
    runtime::connected,
    view::{Attributes, View, class, component, emit, live, view},
};

#[allow(dead_code)]
pub const fn clamp_countdown(value: u16) -> u16 {
    if value > 999 { 999 } else { value }
}

fn countdown_style(value: u16, digits: Option<u8>) -> String {
    match digits.filter(|digits| *digits == 2 || *digits == 3) {
        Some(digits) => format!("--value:{value};--digits:{digits};"),
        None => format!("--value:{value};"),
    }
}

#[component]
pub async fn countdown(
    cx: &Cx,
    value: u16,
    #[default] digits: Option<u8>,
    #[default] live: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let value = clamp_countdown(value);
    Ok(view! {
        <span class=(class!("countdown", attrs.remove("class"))) (attrs)>
            if live {
                (live! {
                    let mut current = value;
                    let initial = emit! {
                        <span
                            style=(countdown_style(current, digits))
                            aria-live="polite"
                            aria-label=(current.to_string())
                        >
                            (current)
                        </span>
                    }?;

                    if !connected(cx) {
                        Ok(initial)
                    } else {
                        while current > 0 {
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            current -= 1;
                            emit! {
                                <span
                                    style=(countdown_style(current, digits))
                                    aria-live="polite"
                                    aria-label=(current.to_string())
                                >
                                    (current)
                                </span>
                            }?;
                        }

                        Ok(initial)
                    }
                })
            } else {
                <span
                    style=(countdown_style(value, digits))
                    aria-live="polite"
                    aria-label=(value.to_string())
                >
                    (value)
                </span>
            }
        </span>
    })
}

#[cfg(test)]
mod tests {
    use super::{clamp_countdown, countdown_style};

    #[test]
    fn countdown_values_are_clamped_to_documented_range() {
        assert_eq!(clamp_countdown(0), 0);
        assert_eq!(clamp_countdown(999), 999);
        assert_eq!(clamp_countdown(1000), 999);
    }

    #[test]
    fn countdown_style_supports_two_and_three_minimum_digits() {
        assert_eq!(countdown_style(7, Some(2)), "--value:7;--digits:2;");
        assert_eq!(countdown_style(7, Some(3)), "--value:7;--digits:3;");
        assert_eq!(countdown_style(7, None), "--value:7;");
        assert_eq!(countdown_style(7, Some(1)), "--value:7;");
        assert_eq!(countdown_style(7, Some(4)), "--value:7;");
    }
}
