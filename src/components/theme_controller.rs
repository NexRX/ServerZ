use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

#[component]
pub async fn theme_controller(
    value: String,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(
        view! { <input type="checkbox" value=(value) class=(class!("theme-controller", attrs.remove("class"))) (attrs)> },
    )
}
