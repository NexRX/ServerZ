use crate::components::*;
use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Child, View, component, view},
};

#[component]
pub async fn app_menu(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="drawer lg:drawer-open">
          <input id="my-drawer-4" type="checkbox" class="drawer-toggle inline" />
          <div class="drawer-content">
            // <!-- Page content here -->
            (child)
          </div>

          <div class="drawer-side is-drawer-close:overflow-visible">
            <label for="my-drawer-4" aria-label="close sidebar" class="drawer-overlay"></label>
            <div class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-14 is-drawer-open:w-64">
              // <!-- Sidebar content here -->
              <ul class="menu w-full grow">
                // <!-- List item -->
                <li>
                  <button class="is-drawer-close:tooltip is-drawer-close:tooltip-right" data-tip="Homepage">
                    // <!-- Home icon -->
                    <span class="iconify solar--home-2-bold-duotone"></span>
                    <span class="is-drawer-close:hidden">"Homepage"</span>
                  </button>
                </li>

                // <!-- List item -->
                <li>
                  <button class="is-drawer-close:tooltip is-drawer-close:tooltip-right" data-tip="Settings">
                    // <!-- Settings icon -->
                    <span class="iconify solar--settings-bold-duotone"></span>
                    <span class="is-drawer-close:hidden">"Settings"</span>
                  </button>
                </li>
              </ul>
            </div>
          </div>
        </div>
    })
}
