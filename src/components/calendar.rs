use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn calendar(
    #[default] min: String,
    #[default] max: String,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! {
            <input
                type="date"
                aria-label="Choose date"
                min=(min)
                max=(max)
                class=(class!("input", attrs.remove("class")))
                (attrs)
            >
        },
    )
}

#[component]
pub async fn calendar_cally(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <calendar-date class=(class!("cally", attrs.remove("class"))) (attrs)>
                (child)
            </calendar-date>
        },
    )
}

/// Cally calendar with a live selected-date output.
#[component]
pub async fn calendar_cally_with_output(
    output_id: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let script = format!(
        "document.currentScript.parentElement.querySelector('calendar-date').addEventListener('change',function(event){{document.getElementById('{}').textContent=event.target.value}})",
        output_id
    );
    Ok(view! {
        <div class=(class!(attrs.remove("class"))) (attrs)>
            <calendar-date class="cally">(child)</calendar-date>
            <output id=(output_id.clone()) aria-live="polite" class="label">
                "No date selected"
            </output>
            <script>(script)</script>
        </div>
    })
}
