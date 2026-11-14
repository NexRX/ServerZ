use crate::components::prelude::*;
use topcoat::{
    Result,
    context::Cx,
    router::request::original_uri,
    view::{Child, View, attributes, class, component, view},
};

#[component]
pub async fn app_menu(cx: &Cx, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="drawer lg:drawer-open">
            <input id="app-menu" type="checkbox" class="drawer-toggle inline" />
            <div class="drawer-content">(child)</div>

            <div class="drawer-side is-drawer-close:overflow-visible">
                <label for="app-menu" aria-label="close sidebar" class="drawer-overlay"></label>
                <div
                    class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-16 is-drawer-open:w-52"
                >
                    // <!-- Sidebar content here -->
                    <ul
                        class="menu w-full grow text-xl text-base-content/75 font-light gap-2"
                    >
                        serverz_branding()
                        divider(
                            color: DividerColor::Accent,
                            attrs: attributes! { class="my-0" }
                        )
                        link(
                            name: "Homepage",
                            icon: tw!("solar--home-2-bold-duotone"),
                            href: "/"
                        )
                        link(
                            name: "Settings",
                            icon: tw!("solar--settings-bold-duotone"),
                            href: "/settings"
                        )
                        divider(
                            color: DividerColor::Accent,
                            attrs: attributes! { class="mt-auto my-0" }
                        )
                        drawer_control()
                    </ul>
                </div>
            </div>
        </div>
    })
}

#[component]
async fn serverz_branding(cx: &Cx) -> Result<impl View> {
    Ok(view!(
        <li class="mx-auto is-drawer-close:bg-primary is-drawer-close:rounded-sm">
            <a
                class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
                data-tip=("ServerZ")
                href="/"
            >
                <span class="is-drawer-close:hidden font-black">"Server"</span>
                <span class="bg-primary rounded-sm text-center font-black w-7 h-7">
                    "Z"
                </span>
            </a>
        </li>
    ))
}

#[component]
async fn link(cx: &Cx, name: &str, icon: &str, href: &str) -> Result<impl View> {
    let path = original_uri(cx).path();
    let is_visiting = path.starts_with(href);

    Ok(view!(
        <li>
            <a
                class=(class!(
                    "is-drawer-close:tooltip is-drawer-close:tooltip-right bg-accent-content/10:hover flex",
                    "bg-primary/80 font-normal" if is_visiting,
                ))
                data-tip=(name)
                href=(href)
            >
                <span
                    class=(class!(
                        "iconify justify-center align-middle is-drawer-close:flex-auto",
                        icon,
                    ))
                ></span>
                <span class="pl-2 text-base is-drawer-close:hidden">(name)</span>
            </a>
        </li>
    ))
}

#[component]
async fn drawer_control(cx: &Cx) -> Result<impl View> {
    Ok(view!(
        <li>
            <label
                for="app-menu"
                class="drawer-button is-drawer-close:tooltip is-drawer-close:tooltip-right"
                data-tip="Menu"
            >
                <span
                    class="iconify solar--chevron-last-bold-duotone is-drawer-open:hidden"
                ></span>

                <span
                    class="iconify solar--chevron-first-bold-duotone is-drawer-close:hidden"
                ></span>
                <span class="is-drawer-close:hidden pl-2 text-base">
                    "Collapse Menu"
                </span>
            </label>
        </li>
    ))
}
