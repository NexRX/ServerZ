use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn otp(
    cx: &Cx,
    #[default] length: u8,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let length = length.clamp(4, 6);
    let script = r#"this.querySelector('input').addEventListener('input',function(){this.closest('.otp').querySelectorAll('[data-otp-digit]').forEach(function(el,i){el.textContent=this.value[i]||''}.bind(this))})"#;
    Ok(view! {
        <label class=(class!("otp", attrs.remove("class"))) (attrs)>
            <span data-otp-digit="true" aria-hidden="true"></span>
            <span data-otp-digit="true" aria-hidden="true"></span>
            <span data-otp-digit="true" aria-hidden="true"></span>
            <span data-otp-digit="true" aria-hidden="true"></span>
            <span data-otp-digit="true" aria-hidden="true"></span>
            <span data-otp-digit="true" aria-hidden="true"></span>
            <input
                type="text"
                autocomplete="one-time-code"
                inputmode="numeric"
                maxlength=(length.to_string())
                pattern=(format!("[0-9]{{{length}}}"))
                required="required"
            >
            <script>
                (format!(
                    "document.currentScript.parentElement.querySelector('input').addEventListener('input',function(){{{script}}})",
                ))
            </script>
        </label>
    })
}
