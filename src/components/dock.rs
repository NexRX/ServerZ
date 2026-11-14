use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Dock sizes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum DockSize {
    #[class = "dock-xs"]
    Xs,
    #[class = "dock-sm"]
    Sm,
    #[default]
    #[class = "dock-md"]
    Md,
    #[class = "dock-lg"]
    Lg,
    #[class = "dock-xl"]
    Xl,
}

#[component]
pub async fn dock(
    cx: &Cx,
    #[default] size: DockSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <nav class=(class!("dock", size.to_class(), attrs.remove("class"))) (attrs)>
            (child)
        </nav>
    })
}

#[component]
pub async fn dock_item(
    cx: &Cx,
    active: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let active_class = if active { "dock-active" } else { "" };
    Ok(view! {
        <div class=(class!("dock-item", active_class, attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

#[component]
pub async fn dock_label(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("dock-label", attrs.remove("class"))) (attrs)>(child)</div>
    })
}
