mod api;
mod components;
mod pages;
mod state;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use pages::*;
use state::{AuthState, SidebarState};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/categories")]
    Categories {},
    #[route("/categories/:id")]
    CategoryDetail { id: u64 },
    #[route("/brands")]
    Brands {},
    #[route("/products")]
    Products {},
    #[route("/products/:id")]
    ProductDetail { id: u64 },
    #[route("/sellers")]
    Sellers {},
    #[route("/sellers/:id")]
    SellerDetail { id: u64 },
    #[route("/stores")]
    Stores {},
    #[route("/orders")]
    Orders {},
    #[route("/orders/:id")]
    OrderDetail { id: u64 },
    #[route("/delivery-zones")]
    DeliveryZones {},
    #[route("/delivery-boys")]
    DeliveryBoys {},
    #[route("/banners")]
    Banners {},
    #[route("/promos")]
    Promos {},
    #[route("/settings")]
    Settings {},
    #[route("/roles")]
    Roles {},
    #[route("/system-users")]
    SystemUsers {},
    #[route("/tax-rates")]
    TaxRates {},
    #[route("/faqs")]
    Faqs {},
    #[route("/notifications")]
    Notifications {},
    #[route("/featured-sections")]
    FeaturedSections {},
    #[route("/system-updates")]
    SystemUpdates {},
    #[route("/seller-withdrawals")]
    SellerWithdrawals {},
    #[route("/pending-products")]
    PendingProducts {},
    #[route("/reviews")]
    Reviews {},
    #[route("/support-tickets")]
    SupportTickets {},
    #[route("/refunds")]
    Refunds {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AuthState::default()));
    use_context_provider(|| Signal::new(SidebarState::default()));

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let nav = use_navigator();

    use_effect(move || {
        if auth.read().is_authenticated() {
            nav.push(Route::Dashboard {});
        } else {
            nav.push(Route::Login {});
        }
    });

    rsx! {
        div { class: "flex items-center justify-center h-screen",
            p { "Loading..." }
        }
    }
}
