use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, component, view},
};

#[component]
pub async fn bg_grid(
    cx: &Cx,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class="setup-grid" aria-hidden="true" (attrs)>(child)</div> })
}
