use crate::components::variants::ModalPlacement;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn modal(
    #[default] id: String,
    #[default] placement: ModalPlacement,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <dialog id=(id) class=(class!("modal", format!("modal-{}", placement.suffix()), attrs.remove("class"))) (attrs)>(child)</dialog> },
    )
}

/// Popover-based modal for cases where focus trapping is not desired.
#[component]
pub async fn modal_popover(
    #[default] id: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div id=(id) popover="popover" class=(class!("modal", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}

#[component]
pub async fn modal_trigger(
    #[default] id: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <button type="button" onclick=(format!("document.getElementById('{}').showModal()", id)) class=(class!("btn", attrs.remove("class"))) (attrs)>(child)</button> },
    )
}

#[component]
pub async fn modal_popover_trigger(
    #[default] id: String,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <button type="button" popovertarget=(id) class=(class!("btn", attrs.remove("class"))) (attrs)>(child)</button> },
    )
}

#[component]
pub async fn modal_box(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("modal-box", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn modal_action(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("modal-action", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn modal_backdrop(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(
        view! { <form method="dialog" class=(class!("modal-backdrop", attrs.remove("class"))) (attrs)><button aria-label="Close">"close"</button></form> },
    )
}
#[component]
pub async fn modal_close(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <button type="submit" class=(class!("btn", attrs.remove("class"))) (attrs)>(child)</button> },
    )
}
