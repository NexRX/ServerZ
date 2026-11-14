use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Table sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TableSize {
    #[class = "table-xs"]
    Xs,
    #[class = "table-sm"]
    Sm,
    #[default]
    #[class = "table-md"]
    Md,
    #[class = "table-lg"]
    Lg,
    #[class = "table-xl"]
    Xl,
}

/// Table styles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum TableStyle {
    #[default]
    #[class = ""]
    Solid,
    #[class = ""]
    Outline,
    #[class = ""]
    Dash,
    #[class = ""]
    Soft,
    #[class = ""]
    Ghost,
}

#[component]
pub async fn table(
    cx: &Cx,
    #[default] size: TableSize,
    #[default] style: TableStyle,
    #[default] zebra: bool,
    #[default] pin_rows: bool,
    #[default] pin_cols: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let z = if zebra { "table-zebra" } else { "" };
    let r = if pin_rows { "table-pin-rows" } else { "" };
    let c = if pin_cols { "table-pin-cols" } else { "" };
    Ok(view! {
        <div class="overflow-x-auto">
            <table
                class=(class!(
                    "table",
                    size.to_class(),
                    style.to_class(),
                    z,
                    r,
                    c,
                    attrs.remove("class"),
                ))
                (attrs)
            >
                (child)
            </table>
        </div>
    })
}
