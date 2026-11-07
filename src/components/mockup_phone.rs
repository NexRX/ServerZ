use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn mockup_phone(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("mockup-phone", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn mockup_phone_camera(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("mockup-phone-camera", attrs.remove("class"))) (attrs)></div>
    })
}

#[component]
pub async fn mockup_phone_display(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! {
            <div class=(class!("mockup-phone-display", attrs.remove("class"))) (attrs)>
                (child)
            </div>
        },
    )
}
