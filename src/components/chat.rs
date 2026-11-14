use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Chat bubble colors matching daisyUI's color system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum ChatBubbleColor {
    #[default]
    #[class = "chat-bubble-neutral"]
    Neutral,
    #[class = "chat-bubble-primary"]
    Primary,
    #[class = "chat-bubble-secondary"]
    Secondary,
    #[class = "chat-bubble-accent"]
    Accent,
    #[class = "chat-bubble-info"]
    Info,
    #[class = "chat-bubble-success"]
    Success,
    #[class = "chat-bubble-warning"]
    Warning,
    #[class = "chat-bubble-error"]
    Error,
}

#[component]
pub async fn chat(
    cx: &Cx,
    #[default] end: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let chat_end = if end { "chat-end" } else { "chat-start" };
    Ok(view! {
        <div class=(class!("chat", chat_end, attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

#[component]
pub async fn chat_bubble(
    cx: &Cx,
    #[default] color: ChatBubbleColor,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!("chat-bubble", color.to_class(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
