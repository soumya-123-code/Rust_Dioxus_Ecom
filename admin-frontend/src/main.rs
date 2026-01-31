mod api;
mod components;
mod pages;
mod state;
mod services;
mod types;
mod utils;
mod hooks;

use dioxus::prelude::*;
use pages::*;
use state::{AuthState, SidebarState, ThemeState};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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
    #[route("/attributes")]
    Attributes {},
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
    #[route("/users")]
    Users {},
    #[route("/users/:id")]
    UserDetail { id: String },
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
    #[route("/commission-config")]
    CommissionConfig {},
    #[route("/search")]
    SearchPage {},
    #[route("/activity-logs")]
    ActivityLogs {},
    #[route("/financial-reports")]
    FinancialReports {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AuthState::default()));
    use_context_provider(|| Signal::new(SidebarState::default()));
    let theme_state = use_context_provider(|| Signal::new(ThemeState::default()));
    
    // Apply initial theme
    use_effect(move || {
        theme_state.read().apply_theme();
    });

    rsx! {
        // Global styles
        document::Link { rel: "stylesheet", href: "/styles/main.css" }
        document::Link { rel: "stylesheet", href: "/styles/utilities.css" }
        document::Link { rel: "stylesheet", href: "/styles/variables.css" }
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
