use crate::components::variants::{ComponentSize, ComponentStyle};
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn table(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class="overflow-x-auto"><table class=(class!("table", attrs.remove("class"))) (attrs)>(child)</table></div> },
    )
}

#[component]
pub async fn table_typed(
    size: ComponentSize,
    style: ComponentStyle,
    zebra: bool,
    pin_rows: bool,
    pin_cols: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let style_class = match style {
        ComponentStyle::Solid => "",
        ComponentStyle::Outline => "",
        ComponentStyle::Dash => "",
        ComponentStyle::Soft => "",
        ComponentStyle::Ghost => "",
    };
    let z = if zebra { "table-zebra" } else { "" };
    let r = if pin_rows { "table-pin-rows" } else { "" };
    let c = if pin_cols { "table-pin-cols" } else { "" };
    Ok(
        view! { <div class="overflow-x-auto"><table class=(class!("table", format!("table-{}", size.suffix()), style_class, z, r, c, attrs.remove("class"))) (attrs)>(child)</table></div> },
    )
}
