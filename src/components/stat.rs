use server_z_macros::TailwindClass;
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

/// Stats directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, TailwindClass)]
pub enum StatsDirection {
    #[default]
    #[class = "stats-horizontal"]
    Horizontal,
    #[class = "stats-vertical"]
    Vertical,
}

#[component]
pub async fn stats(
    cx: &Cx,
    direction: StatsDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!("stats", direction.to_class(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

#[component]
pub async fn stat(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("stat", attrs.remove("class"))) (attrs)>(child)</div> })
}

#[component]
pub async fn stat_title(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("stat-desc", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn stat_value(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("stat-value", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn stat_desc(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("stat-desc", attrs.remove("class"))) (attrs)>(child)</div>
    })
}
