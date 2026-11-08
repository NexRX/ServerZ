use server_z_macros::TailwindClass;
use topcoat::{
    Result,
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
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("stat", attrs.remove("class"))) (attrs)>(child)</div> })
}

#[component]
pub async fn stat_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("stat-desc", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn stat_value(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("stat-value", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn stat_desc(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("stat-desc", attrs.remove("class"))) (attrs)>(child)</div>
    })
}
