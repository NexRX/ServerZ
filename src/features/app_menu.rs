use crate::components::*;
use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Child, View, component, view},
};

#[component]
pub async fn app_menu(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="drawer lg:drawer-open min-w-10 is-drawer-close:min-w-10 is-drawer-close:visible">
          <input id="app-menu" type="checkbox" class="drawer-toggle" />
          <div class="drawer-content flex flex-col items-center justify-center">
            // <!-- Page content here -->
            <label for="app-menu" class="btn drawer-button lg:hidden">
              "Open drawer"
            </label>
          </div>
          <div class="drawer-side">
            <label for="app-menu" aria-label="close sidebar" class="drawer-overlay"></label>
            <ul class="menu bg-base-200 min-h-full w-80 p-4">
              // <!-- Sidebar content here -->
              <li><a>"Sidebar Item 1"</a></li>
              <li><a>"Sidebar Item 2"</a></li>
            </ul>
          </div>
        </div>
    })
}
