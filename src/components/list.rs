use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn list(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <ul class=(class!("list", attrs.remove("class"))) (attrs)>(child)</ul> })
}
#[component]
pub async fn list_row(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li class=(class!("list-row", attrs.remove("class"))) (attrs)>(child)</li>
    })
}
#[component]
pub async fn list_col_grow(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("list-col-grow", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}
#[component]
pub async fn list_col_wrap(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("list-col-wrap", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}
