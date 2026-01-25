use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::state::{AuthState, SidebarState};
use crate::Route;

#[component]
pub fn Header() -> Element {
    let mut auth = use_context::<Signal<AuthState>>();
    let mut sidebar_state = use_context::<Signal<SidebarState>>();
    let nav = use_navigator();

    let user_name = auth.read().user.as_ref().map(|u| u.name.clone()).unwrap_or_default();

    let handle_logout = move |_| {
        auth.write().logout();
        nav.push(Route::Login {});
    };

    let toggle_sidebar = move |_| {
        sidebar_state.write().toggle();
    };

    rsx! {
        header { class: "bg-white shadow-sm h-16 flex items-center justify-between px-6 fixed top-0 md:left-64 left-0 right-0 z-10 transition-all duration-300",
            div { class: "flex items-center",
                button {
                    class: "mr-4 text-gray-500 hover:text-gray-700 focus:outline-none md:hidden",
                    onclick: toggle_sidebar,
                    span { class: "text-2xl", "☰" }
                }
                h2 { class: "text-lg font-semibold text-gray-800", "Admin Panel" }
            }
            div { class: "flex items-center space-x-4",
                span { class: "text-gray-600", "{user_name}" }
                button {
                    class: "px-4 py-2 text-sm text-red-600 hover:bg-red-50 rounded transition-colors",
                    onclick: handle_logout,
                    "Logout"
                }
            }
        }
    }
}
