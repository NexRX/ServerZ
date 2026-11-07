use crate::components::prelude::*;
use topcoat::{
    Result,
    view::{Child, View, class, component, view},
};

#[component]
pub async fn app_menu(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="drawer lg:drawer-open">
            <input id="my-drawer-4" type="checkbox" class="drawer-toggle inline" />
            <div class="drawer-content">(child)</div>

            <div class="drawer-side is-drawer-close:overflow-visible">
                <label
                    for="my-drawer-4"
                    aria-label="close sidebar"
                    class="drawer-overlay"
                ></label>
                <div
                    class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-14 is-drawer-open:w-64"
                >
                    // <!-- Sidebar content here -->
                    <ul class="menu w-full grow">
                        entry(
                            name: "Homepage",
                            icon: "solar--home-2-bold-duotone",
                            href: "/"
                        )
                        entry(
                            name: "Settings",
                            icon: "solar--settings-bold-duotone",
                            href: "/settings"
                        )
                        divider(color: DividerColor::Accent)
                    </ul>
                </div>
            </div>
        </div>
    })
}

#[component]
async fn entry(name: &str, icon: &str, href: &str) -> Result<impl View> {
    Ok(view!(
        <li>
            <a
                class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
                data-tip=(name)
                href=(href)
            >
                <span class=(class!("iconify", icon))></span>
                <span class="is-drawer-close:hidden">(name)</span>
            </a>
        </li>
    ))
}
