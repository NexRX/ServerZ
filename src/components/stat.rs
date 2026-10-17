use crate::components::variants::ComponentDirection;
use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn stats(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("stats", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn stats_typed(
    direction: ComponentDirection,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div class=(class!("stats", format!("stats-{}", direction.suffix()), attrs.remove("class"))) (attrs)>(child)</div> },
    )
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
    Ok(view! { <div class=(class!("stat-title", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn stat_value(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("stat-value", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn stat_desc(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("stat-desc", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn stat_figure(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("stat-figure", attrs.remove("class"))) (attrs)>(child)</div> })
}
#[component]
pub async fn stat_actions(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!("stat-actions", attrs.remove("class"))) (attrs)>(child)</div> })
}
