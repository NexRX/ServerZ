use crate::components::variants::ComponentColor;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn chat(
    #[default] end: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let placement = if end {
        "chat chat-end"
    } else {
        "chat chat-start"
    };
    Ok(view! { <div class=(class!(placement, attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn chat_bubble(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("chat-bubble", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn chat_bubble_typed(
    color: ComponentColor,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("chat-bubble", format!("chat-bubble-{}", color.suffix()), attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
#[component]
pub async fn chat_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("chat-header", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn chat_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("chat-footer", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn chat_image(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("chat-image avatar", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
