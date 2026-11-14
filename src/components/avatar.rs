use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, class, component, view},
};

#[component]
pub async fn avatar(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("avatar", attrs.remove("class"))) (attrs)>(child)</div>
    })
}

#[component]
pub async fn avatar_group(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!("avatar-group -space-x-4", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

#[component]
pub async fn avatar_image(
    cx: &Cx,
    src: String,
    #[default] alt: String,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(attrs.remove("class"))) (attrs)>
            <img src=(src) alt=(alt)>
        </div>
    })
}

#[component]
pub async fn avatar_placeholder(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!("avatar avatar-placeholder", attrs.remove("class")))
            (attrs)
        >
            <div class="bg-neutral text-neutral-content w-12">
                <span>(child)</span>
            </div>
        </div>
    })
}

#[component]
pub async fn avatar_online(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("avatar avatar-online", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

#[component]
pub async fn avatar_offline(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("avatar avatar-offline", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}
