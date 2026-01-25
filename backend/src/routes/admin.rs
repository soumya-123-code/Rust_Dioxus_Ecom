use axum::{
    routing::{get, post, put, delete, patch},
    Router,
};
use crate::controllers;
use crate::utils::types::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        // Dashboard routes
        .route("/dashboard/stats", get(controllers::dashboard::get_stats))
        .route("/dashboard/chart-data", get(controllers::dashboard::get_chart_data))
        
        // Category routes
        .route("/categories", get(controllers::categories::list))
        .route("/categories", post(controllers::categories::create))
        .route("/categories/datatable", get(controllers::categories::datatable))
        .route("/categories/search", get(controllers::categories::search))
        .route("/categories/:id", get(controllers::categories::show))
        .route("/categories/:id", put(controllers::categories::update))
        .route("/categories/:id", delete(controllers::categories::delete))
        
        // Brand routes
        .route("/brands", get(controllers::brands::list))
        .route("/brands", post(controllers::brands::create))
        .route("/brands/datatable", get(controllers::brands::datatable))
        .route("/brands/search", get(controllers::brands::search))
        .route("/brands/:id", get(controllers::brands::show))
        .route("/brands/:id", put(controllers::brands::update))
        .route("/brands/:id", delete(controllers::brands::delete))
        
        // Product routes
        .route("/products", get(controllers::products::list))
        .route("/products/datatable", get(controllers::products::datatable))
        .route("/products/search", get(controllers::products::search))
        .route("/products/:id", get(controllers::products::show))
        .route("/products/:id/verification-status", post(controllers::products::update_verification_status))
        .route("/products/:id/update-status", post(controllers::products::update_status))
        
        // Seller routes
        .route("/sellers", get(controllers::sellers::list))
        .route("/sellers/datatable", get(controllers::sellers::datatable))
        .route("/sellers/:id", get(controllers::sellers::show))
        .route("/sellers/:id/verification-status", post(controllers::sellers::update_verification_status))
        
        // Store routes
        .route("/stores", get(controllers::stores::list))
        .route("/stores/datatable", get(controllers::stores::datatable))
        .route("/stores/:id", get(controllers::stores::show))
        .route("/stores/:id/verification-status", post(controllers::stores::update_verification_status))
        
        // Order routes
        .route("/orders", get(controllers::orders::list))
        .route("/orders/datatable", get(controllers::orders::datatable))
        .route("/orders/:id", get(controllers::orders::show))
        .route("/orders/:id/:status", post(controllers::orders::update_status))
        
        // Delivery Zone routes
        .route("/delivery-zones", get(controllers::delivery_zones::list))
        .route("/delivery-zones", post(controllers::delivery_zones::create))
        .route("/delivery-zones/datatable", get(controllers::delivery_zones::datatable))
        .route("/delivery-zones/:id", get(controllers::delivery_zones::show))
        .route("/delivery-zones/:id", put(controllers::delivery_zones::update))
        .route("/delivery-zones/:id", delete(controllers::delivery_zones::delete))
        
        // Delivery Boy routes
        .route("/delivery-boys", get(controllers::delivery_boys::list))
        .route("/delivery-boys/datatable", get(controllers::delivery_boys::datatable))
        .route("/delivery-boys/:id", get(controllers::delivery_boys::show))
        .route("/delivery-boys/:id/verification-status", post(controllers::delivery_boys::update_verification_status))
        
        // Banner routes
        .route("/banners", get(controllers::banners::list))
        .route("/banners", post(controllers::banners::create))
        .route("/banners/datatable", get(controllers::banners::datatable))
        .route("/banners/:id", get(controllers::banners::show))
        .route("/banners/:id", put(controllers::banners::update))
        .route("/banners/:id", delete(controllers::banners::delete))
        
        // Promo routes
        .route("/promos", get(controllers::promos::list))
        .route("/promos", post(controllers::promos::create))
        .route("/promos/datatable", get(controllers::promos::datatable))
        .route("/promos/:id", get(controllers::promos::show))
        .route("/promos/:id", put(controllers::promos::update))
        .route("/promos/:id", delete(controllers::promos::delete))
        
        // Settings routes
        .route("/settings", get(controllers::settings::list))
        .route("/settings/:variable", get(controllers::settings::show))
        .route("/settings", post(controllers::settings::store))
        
        // Profile routes
        .route("/profile", get(controllers::profile::show))
        .route("/profile", put(controllers::profile::update))
        .route("/profile/password", post(controllers::profile::change_password))
        
        // Role & Permission routes
        .route("/roles", get(controllers::roles::list))
        .route("/roles", post(controllers::roles::create))
        .route("/roles/:id", get(controllers::roles::show))
        .route("/roles/:id", put(controllers::roles::update))
        .route("/roles/:id", delete(controllers::roles::delete))
        .route("/roles/:id/permissions", get(controllers::permissions::list_role_permissions))
        .route("/permissions", get(controllers::permissions::list))
        .route("/permissions/assign", post(controllers::permissions::store))
        
        // System Users routes
        .route("/system-users", get(controllers::system_users::list))
        .route("/system-users", post(controllers::system_users::create))
        .route("/system-users/:id", get(controllers::system_users::show))
        .route("/system-users/:id", put(controllers::system_users::update))
        .route("/system-users/:id", delete(controllers::system_users::delete))
        
        // Tax Class routes
        .route("/tax-classes", get(controllers::tax_classes::list))
        .route("/tax-classes", post(controllers::tax_classes::create))
        .route("/tax-classes/:id", get(controllers::tax_classes::show))
        .route("/tax-classes/:id", put(controllers::tax_classes::update))
        .route("/tax-classes/:id", delete(controllers::tax_classes::delete))
        
        // Tax Rate routes
        .route("/tax-rates", get(controllers::tax_rates::list))
        .route("/tax-rates", post(controllers::tax_rates::create))
        .route("/tax-rates/:id", get(controllers::tax_rates::show))
        .route("/tax-rates/:id", put(controllers::tax_rates::update))
        .route("/tax-rates/:id", delete(controllers::tax_rates::delete))
        
        // FAQ routes
        .route("/faqs", get(controllers::faqs::list))
        .route("/faqs", post(controllers::faqs::create))
        .route("/faqs/:id", get(controllers::faqs::show))
        .route("/faqs/:id", put(controllers::faqs::update))
        .route("/faqs/:id", delete(controllers::faqs::delete))
        
        // Featured Section routes
        .route("/featured-sections", get(controllers::featured_sections::list))
        .route("/featured-sections", post(controllers::featured_sections::create))
        .route("/featured-sections/:id", get(controllers::featured_sections::show))
        .route("/featured-sections/:id", put(controllers::featured_sections::update))
        .route("/featured-sections/:id", delete(controllers::featured_sections::delete))
        
        // Notification routes
        .route("/notifications", get(controllers::notifications::list))
        .route("/notifications", post(controllers::notifications::send))
        
        // Commission & Withdrawal routes
        .route("/commissions", get(controllers::commissions::list))
        .route("/commissions/:id/settle", post(controllers::commissions::settle))
        .route("/seller-withdrawals", get(controllers::withdrawals::seller_list))
        .route("/seller-withdrawals/:id", patch(controllers::withdrawals::update_seller_withdrawal))
        .route("/delivery-boy-withdrawals", get(controllers::withdrawals::delivery_boy_list))
        .route("/delivery-boy-withdrawals/:id", patch(controllers::withdrawals::update_delivery_boy_withdrawal))

        // System Update routes
        .route("/system-updates", get(controllers::system_updates::list))
        .route("/system-updates", post(controllers::system_updates::store))

        // Review routes
        .route("/reviews", get(controllers::reviews::list))
        .route("/reviews/:id/status", post(controllers::reviews::update_status))
        .route("/reviews/:id", delete(controllers::reviews::delete))

        // Support Ticket routes
        .route("/support-tickets", get(controllers::support_tickets::list))
        .route("/support-tickets/:id", get(controllers::support_tickets::show))
        .route("/support-tickets/:id/reply", post(controllers::support_tickets::reply))
        .route("/support-tickets/:id/status", post(controllers::support_tickets::update_status))

        // Refund routes
        .route("/refunds", get(controllers::refunds::list))
        .route("/refunds/:id/status", post(controllers::refunds::update_status))
}
