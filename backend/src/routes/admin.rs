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
        .route("/dashboard/extended-stats", get(controllers::dashboard::get_extended_stats))
        .route("/dashboard/order-breakdown", get(controllers::dashboard::get_order_status_breakdown))
        .route("/dashboard/top-products", get(controllers::dashboard::get_top_products))
        .route("/dashboard/top-categories", get(controllers::dashboard::get_top_categories))
        .route("/dashboard/vendor-performance", get(controllers::dashboard::get_vendor_performance))
        .route("/dashboard/recent-activities", get(controllers::dashboard::get_recent_activities))
        
        // Category routes
        .route("/categories", get(controllers::categories::list))
        .route("/categories", post(controllers::categories::create))
        .route("/categories/datatable", get(controllers::categories::datatable))
        .route("/categories/search", get(controllers::categories::search))
        .route("/categories/:id/status", put(controllers::categories::update_status))
        .route("/categories/:id", get(controllers::categories::show))
        .route("/categories/:id", put(controllers::categories::update))
        .route("/categories/:id", delete(controllers::categories::delete))
        
        // Brand routes
        .route("/brands", get(controllers::brands::list))
        .route("/brands", post(controllers::brands::create))
        .route("/brands/datatable", get(controllers::brands::datatable))
        .route("/brands/search", get(controllers::brands::search))
        .route("/brands/:id/status", put(controllers::brands::update_status))
        .route("/brands/:id", get(controllers::brands::show))
        .route("/brands/:id", put(controllers::brands::update))
        .route("/brands/:id", delete(controllers::brands::delete))
        
        // Attribute routes
        .route("/attributes", get(controllers::attributes::list))
        .route("/attributes", post(controllers::attributes::create))
        .route("/attributes/datatable", get(controllers::attributes::datatable))
        .route("/attributes/:id/values", get(controllers::attributes::list_values))
        .route("/attributes/:id/values", post(controllers::attributes::create_value))
        .route("/attributes/:id/values/:value_id", delete(controllers::attributes::delete_value))
        .route("/attributes/:id", get(controllers::attributes::show))
        .route("/attributes/:id", put(controllers::attributes::update))
        .route("/attributes/:id", delete(controllers::attributes::delete))

        // Product routes
        .route("/products", get(controllers::products::list))
        .route("/products", post(controllers::products::create))
        .route("/products/datatable", get(controllers::products::datatable))
        .route("/products/search", get(controllers::products::search))
        .route("/products/:id", get(controllers::products::show))
        .route("/products/:id", put(controllers::products::update))
        .route("/products/:id", delete(controllers::products::delete))
        .route("/products/:id/status", put(controllers::products::update_status))
        .route("/products/:id/verification", put(controllers::products::update_verification_status))
        // Legacy routes for backward compatibility
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
        .route("/orders/stats", get(controllers::orders::get_order_stats))
        .route("/orders/bulk-update", post(controllers::orders::bulk_update_status))
        .route("/orders/:id", get(controllers::orders::show))
        .route("/orders/:id/timeline", get(controllers::orders::get_order_timeline))
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

        // Coupon routes
        .route("/coupons", get(controllers::coupons::list))
        .route("/coupons", post(controllers::coupons::create))
        .route("/coupons/datatable", get(controllers::coupons::datatable))
        .route("/coupons/search", get(controllers::coupons::search))
        .route("/coupons/:id", get(controllers::coupons::show))
        .route("/coupons/:id", put(controllers::coupons::update))
        .route("/coupons/:id", delete(controllers::coupons::delete))

        // Pages/CMS routes
        .route("/pages", get(controllers::pages::list))
        .route("/pages", post(controllers::pages::create))
        .route("/pages/datatable", get(controllers::pages::datatable))
        .route("/pages/search", get(controllers::pages::search))
        .route("/pages/slug/:slug", get(controllers::pages::show_by_slug))
        .route("/pages/:id", get(controllers::pages::show))
        .route("/pages/:id", put(controllers::pages::update))
        .route("/pages/:id", delete(controllers::pages::delete))

        // Promo routes - COMMENTED OUT (no promos table in new schema)
        // .route("/promos", get(controllers::promos::list))
        // .route("/promos", post(controllers::promos::create))
        // .route("/promos/datatable", get(controllers::promos::datatable))
        // .route("/promos/:id", get(controllers::promos::show))
        // .route("/promos/:id", put(controllers::promos::update))
        // .route("/promos/:id", delete(controllers::promos::delete))
        
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
        
        // Users routes (alias for system-users for frontend compatibility)
        .route("/users", get(controllers::system_users::list))
        .route("/users", post(controllers::system_users::create))
        .route("/users/datatable", get(controllers::system_users::datatable))
        .route("/users/:id", get(controllers::system_users::show))
        .route("/users/:id", put(controllers::system_users::update))
        .route("/users/:id/status", put(controllers::system_users::update_status))
        .route("/users/:id", delete(controllers::system_users::delete))

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

        // Commission Configuration routes
        .route("/commission-config", get(controllers::commission_config::get_all_commission_configs))
        .route("/commission-config/vendor/:id", get(controllers::commission_config::get_vendor_commission))
        .route("/commission-config/vendor", post(controllers::commission_config::upsert_vendor_commission))
        .route("/commission-config/global", get(controllers::commission_config::get_global_settings))
        .route("/commission-config/global", put(controllers::commission_config::update_global_settings))
        .route("/commission-config/history", get(controllers::commission_config::get_commission_history))
        .route("/commission-config/summary", get(controllers::commission_config::get_commission_summary))

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

        // Global Search routes
        .route("/search", get(controllers::global_search::global_search))
        .route("/search/suggestions", get(controllers::global_search::search_suggestions))
        .route("/search/entity", get(controllers::global_search::search_entity))
        .route("/search/popular", get(controllers::global_search::get_popular_searches))

        // Activity Logs routes
        .route("/activity-logs", get(controllers::activity_logs::list_activity_logs))
        .route("/activity-logs/summary", get(controllers::activity_logs::get_activity_summary))
        .route("/activity-logs/chart", get(controllers::activity_logs::get_activity_chart))
        .route("/activity-logs/filters", get(controllers::activity_logs::get_filter_options))
        .route("/activity-logs/:id", get(controllers::activity_logs::get_activity_log))
        .route("/activity-logs", post(controllers::activity_logs::log_activity))
        
        // Financial Reports routes
        .route("/reports/financial/overview", get(controllers::financial_reports::get_financial_overview))
        .route("/reports/financial/sales", get(controllers::financial_reports::get_sales_report))
        .route("/reports/financial/revenue-breakdown", get(controllers::financial_reports::get_revenue_breakdown))
        .route("/reports/financial/commission", get(controllers::financial_reports::get_commission_report))
        .route("/reports/financial/top-performers", get(controllers::financial_reports::get_top_performers))
        .route("/reports/financial/payment-analysis", get(controllers::financial_reports::get_payment_analysis))
        .route("/reports/financial/export", get(controllers::financial_reports::export_financial_data))
}
