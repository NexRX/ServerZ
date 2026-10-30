use crate::components::variants::{ComponentColor, TooltipAlignment, TooltipPlacement};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn tooltip(
    text: String,
    #[default] color: ComponentColor,
    #[default] placement: TooltipPlacement,
    #[default] alignment: TooltipAlignment,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let open_class = if open { "tooltip-open" } else { "" };
    Ok(
        view! { <div data-tip=(text) class=(class!("tooltip", format!("tooltip-{}", color.suffix()), format!("tooltip-{}", placement.suffix()), format!("tooltip-{}", alignment.suffix()), open_class, attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
