use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Modal placements.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum ModalPlacement {
    #[default]
    #[class = "modal-middle"]
    Middle,
    #[class = "modal-top"]
    Top,
    #[class = "modal-bottom"]
    Bottom,
    #[class = "modal-start"]
    Start,
    #[class = "modal-end"]
    End,
}

#[component]
pub async fn modal(
    cx: &Cx,
    #[default] id: String,
    #[default] placement: ModalPlacement,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <dialog
            id=(id)
            class=(class!("modal", placement.to_class(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </dialog>
    })
}
