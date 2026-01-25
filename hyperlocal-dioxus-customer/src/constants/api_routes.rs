// API Routes constants
pub struct ApiRoutes;

impl ApiRoutes {
    pub const BASE_URL: &'static str = "http://localhost:8080/api/client";
    
    // Auth routes
    pub const LOGIN: &'static str = "login";
    pub const REGISTER: &'static str = "register";
    pub const VERIFY_USER: &'static str = "verify-user";
    pub const FORGOT_PASSWORD: &'static str = "forget-password";
    pub const LOGOUT: &'static str = "logout";
    
    // Product routes
    pub const CATEGORIES: &'static str = "categories";
    pub const BANNERS: &'static str = "banners";
    pub const PRODUCTS: &'static str = "delivery-zone/products";
    pub const PRODUCT_DETAIL: &'static str = "products";
    
    // Cart routes
    pub const CART: &'static str = "user/cart";
    pub const ADD_TO_CART: &'static str = "user/cart/add";
    pub const REMOVE_FROM_CART: &'static str = "user/cart/item";
    pub const CLEAR_CART: &'static str = "user/cart/clear-cart";
    
    // Order routes
    pub const ORDERS: &'static str = "user/orders";
    pub const ORDER_DETAIL: &'static str = "user/orders";
    
    // Address routes
    pub const ADDRESSES: &'static str = "user/addresses";
    
    // Wallet routes
    pub const WALLET: &'static str = "user/wallet";
    pub const WALLET_TRANSACTIONS: &'static str = "user/wallet/transactions";
    pub const PREPARE_RECHARGE: &'static str = "user/wallet/prepare-wallet-recharge";
}
