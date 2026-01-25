use dioxus::prelude::*;

use crate::pages::*;
use crate::context::{AuthContext, CartContext, ThemeContext};

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Splash {},
    
    #[route("/intro-slider")]
    IntroSlider {},
    
    #[route("/login")]
    Login {},
    
    #[route("/register")]
    Register {},
    
    #[route("/otp-verification")]
    OtpVerification {},
    
    #[route("/home")]
    Home {},
    
    #[route("/categories")]
    Categories {},
    
    #[route("/cart")]
    Cart {},
    
    #[route("/product-listing")]
    ProductListing {},
    
    #[route("/product-detail/:slug")]
    ProductDetail { slug: String },
    
    #[route("/account")]
    Account {},
    
    #[route("/address-list")]
    AddressList {},
    
    #[route("/payment-options")]
    PaymentOptions {},
    
    #[route("/my-orders")]
    MyOrders {},
    
    #[route("/order-detail/:slug")]
    OrderDetail { slug: String },
    
    #[route("/wallet")]
    Wallet {},
    
    #[route("/wishlist")]
    Wishlist {},
    
    #[route("/search")]
    Search {},
    
    #[route("/user-profile")]
    UserProfile {},
    
    #[route("/near-by-store")]
    NearByStores {},
    
    #[route("/near-by-store-details/:slug")]
    NearByStoreDetails { slug: String },
    
    #[route("/..")]
    NotFound {},
}

#[component]
pub fn NotFound() -> Element {
    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center bg-gray-50",
            div {
                class: "text-center",
                h1 {
                    class: "text-4xl font-bold text-gray-900 mb-4",
                    "404"
                }
                p {
                    class: "text-gray-600 mb-4",
                    "Page not found"
                }
                Link {
                    to: Route::Home {},
                    class: "text-indigo-600 hover:text-indigo-800",
                    "Go to Home"
                }
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    // Initialize context providers (React-like)
    use_context_provider(|| Signal::new(AuthContext::default()));
    use_context_provider(|| Signal::new(CartContext::default()));
    use_context_provider(|| Signal::new(ThemeContext::default()));

    rsx! {
        Router::<Route> {}
    }
}
