use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Join directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum JoinDirection {
    #[default]
    #[class = "join-horizontal"]
    Horizontal,
    #[class = "join-vertical"]
    Vertical,
}

#[component]
pub async fn join(
    #[default] direction: JoinDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!("join", direction.to_class(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
