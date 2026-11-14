use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Footer directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum FooterDirection {
    #[default]
    #[class = "footer-horizontal"]
    Horizontal,
    #[class = "footer-vertical"]
    Vertical,
}

#[component]
pub async fn footer(
    cx: &Cx,
    #[default] direction: FooterDirection,
    #[default] center: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let placement = if center { "footer-center" } else { "" };
    Ok(view! {
        <footer
            class=(class!(
                "footer",
                direction.to_class(),
                placement,
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </footer>
    })
}
