use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Megamenu layouts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum MegamenuLayout {
    #[default]
    #[class = ""]
    Normal,
    #[class = "megamenu-wide"]
    Wide,
    #[class = "megamenu-full"]
    Full,
    #[class = "megamenu-vertical"]
    Vertical,
}

#[component]
pub async fn megamenu(
    cx: &Cx,
    layout: MegamenuLayout,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav
            class=(class!("megamenu", layout.to_class(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </nav>
    })
}
