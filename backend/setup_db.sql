-- Users table
CREATE TABLE IF NOT EXISTS users (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    email_verified_at TIMESTAMP NULL,
    password VARCHAR(255) NOT NULL,
    remember_token VARCHAR(100) NULL,
    mobile VARCHAR(20) NOT NULL,
    referral_code VARCHAR(32) NULL,
    friends_code VARCHAR(32) NULL,
    reward_points DECIMAL(10, 2) NOT NULL DEFAULT 0.00,
    status VARCHAR(10) NOT NULL DEFAULT 'active',
    access_panel VARCHAR(20) NULL,
    iso_2 VARCHAR(5) NULL,
    country VARCHAR(100) NULL,
    deleted_at TIMESTAMP NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id VARCHAR(255) PRIMARY KEY,
    user_id BIGINT UNSIGNED NULL,
    ip_address VARCHAR(45) NULL,
    user_agent TEXT NULL,
    payload LONGTEXT NOT NULL,
    last_activity INT NOT NULL
);

-- Roles table
CREATE TABLE IF NOT EXISTS roles (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    team_id BIGINT UNSIGNED NULL,
    name VARCHAR(255) NOT NULL,
    guard_name VARCHAR(255) NOT NULL,
    label VARCHAR(100) NULL,
    description VARCHAR(255) NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Permissions table
CREATE TABLE IF NOT EXISTS permissions (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    guard_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Model Has Roles
CREATE TABLE IF NOT EXISTS model_has_roles (
    role_id BIGINT UNSIGNED NOT NULL,
    model_type VARCHAR(255) NOT NULL,
    model_id BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (role_id, model_id, model_type)
);

-- Settings table
CREATE TABLE IF NOT EXISTS settings (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    variable VARCHAR(100) NOT NULL,
    value LONGTEXT NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Categories table
CREATE TABLE IF NOT EXISTS categories (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL,
    parent_id BIGINT UNSIGNED NULL,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    image VARCHAR(255) NOT NULL,
    banner VARCHAR(255) NULL,
    description TEXT NOT NULL,
    status VARCHAR(10) NOT NULL DEFAULT 'active',
    requires_approval BOOLEAN NOT NULL DEFAULT 0,
    commission DECIMAL(10, 2) NULL,
    background_type VARCHAR(20) NULL,
    background_color VARCHAR(20) NULL,
    font_color VARCHAR(20) NULL,
    metadata JSON NOT NULL,
    deleted_at TIMESTAMP NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Brands table
CREATE TABLE IF NOT EXISTS brands (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    image VARCHAR(255) NOT NULL,
    banner VARCHAR(255) NULL,
    status VARCHAR(1) NOT NULL DEFAULT '1',
    is_featured BOOLEAN NULL,
    metadata JSON NOT NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Products table
CREATE TABLE IF NOT EXISTS products (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL,
    seller_id BIGINT UNSIGNED NOT NULL,
    category_id BIGINT UNSIGNED NOT NULL,
    brand_id BIGINT UNSIGNED NULL,
    product_condition_id BIGINT UNSIGNED NOT NULL,
    provider VARCHAR(255) NULL,
    provider_product_id BIGINT UNSIGNED NULL,
    slug VARCHAR(500) NOT NULL,
    title VARCHAR(255) NOT NULL,
    product_identity INT NULL,
    type VARCHAR(20) NOT NULL,
    short_description VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    indicator VARCHAR(10) NULL,
    download_allowed VARCHAR(1) NOT NULL DEFAULT '0',
    download_link VARCHAR(255) NULL,
    minimum_order_quantity INT NOT NULL DEFAULT 1,
    quantity_step_size INT NOT NULL DEFAULT 1,
    total_allowed_quantity INT NOT NULL,
    is_inclusive_tax VARCHAR(1) NOT NULL DEFAULT '0',
    hsn_code VARCHAR(50) NULL,
    is_returnable VARCHAR(1) NOT NULL DEFAULT '0',
    returnable_days INT NULL,
    is_cancelable VARCHAR(1) NOT NULL DEFAULT '0',
    cancelable_till VARCHAR(50) NULL,
    is_attachment_required VARCHAR(1) NOT NULL DEFAULT '0',
    base_prep_time INT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    verification_status VARCHAR(20) NULL,
    rejection_reason TEXT NULL,
    featured VARCHAR(1) NOT NULL DEFAULT '0',
    requires_otp BOOLEAN NULL,
    video_type VARCHAR(20) NULL,
    video_link VARCHAR(255) NULL,
    cloned_from_id BIGINT UNSIGNED NULL,
    tags TEXT NOT NULL,
    warranty_period VARCHAR(255) NULL,
    guarantee_period VARCHAR(255) NULL,
    made_in VARCHAR(255) NULL,
    image_fit VARCHAR(20) NULL,
    metadata JSON NOT NULL,
    deleted_at TIMESTAMP NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    notification_type VARCHAR(50) NOT NULL,
    target_type VARCHAR(255) NULL,
    target_id BIGINT UNSIGNED NULL,
    data JSON NULL,
    read_at TIMESTAMP NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Insert Default Admin User
-- Password: password (hashed with Argon2)
INSERT INTO users (name, email, password, mobile, status, access_panel, reward_points) 
VALUES ('Admin', 'admin@admin.com', '$argon2id$v=19$m=19456,t=2,p=1$F1DwBEiObimzlbFpoHSMyA$TBV1zmmb6GSFSYVXTB+9PxSHCcofjcLoCVLrb7MRaLc', '1234567890', 'active', 'admin', 0.00)
ON DUPLICATE KEY UPDATE email=email;

-- Delivery Zones
CREATE TABLE IF NOT EXISTS delivery_zones (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    center_latitude DECIMAL(10, 8) NOT NULL,
    center_longitude DECIMAL(11, 8) NOT NULL,
    radius_km DOUBLE NOT NULL,
    boundary_json JSON NULL,
    status VARCHAR(10) NOT NULL DEFAULT 'active',
    delivery_time_per_km INT NULL,
    buffer_time INT NULL,
    min_order_amount DECIMAL(10, 2) NULL,
    delivery_charge_type VARCHAR(255) NULL,
    base_delivery_charge DECIMAL(10, 2) NULL,
    delivery_charge_per_km DECIMAL(10, 2) NULL,
    free_delivery_above DECIMAL(10, 2) NULL,
    per_order_earning DECIMAL(10, 2) NULL,
    per_km_earning DECIMAL(10, 2) NULL,
    base_earning DECIMAL(10, 2) NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Delivery Boys
CREATE TABLE IF NOT EXISTS delivery_boys (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT UNSIGNED NOT NULL,
    delivery_zone_id BIGINT UNSIGNED NULL,
    address VARCHAR(255) NOT NULL,
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    zipcode VARCHAR(20) NOT NULL,
    country VARCHAR(100) NOT NULL,
    national_identity_card TEXT NOT NULL,
    driving_license TEXT NOT NULL,
    vehicle_registration TEXT NULL,
    vehicle_number VARCHAR(255) NULL,
    vehicle_type VARCHAR(255) NULL,
    verification_status VARCHAR(20) NOT NULL DEFAULT 'not_approved',
    metadata JSON NULL,
    availability_status VARCHAR(20) NOT NULL DEFAULT 'offline',
    deleted_at TIMESTAMP NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Promos
CREATE TABLE IF NOT EXISTS promos (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    code VARCHAR(50) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NULL,
    discount_type VARCHAR(50) NOT NULL,
    discount_value DECIMAL(10, 2) NOT NULL,
    min_order_amount DECIMAL(10, 2) NULL,
    max_discount_amount DECIMAL(10, 2) NULL,
    usage_limit INT NULL,
    usage_limit_per_user INT NULL,
    times_used INT NOT NULL DEFAULT 0,
    start_date TIMESTAMP NULL,
    end_date TIMESTAMP NULL,
    status VARCHAR(10) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Product Conditions
CREATE TABLE IF NOT EXISTS product_conditions (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    description TEXT NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Sellers
CREATE TABLE IF NOT EXISTS sellers (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT UNSIGNED NOT NULL,
    address VARCHAR(255) NOT NULL,
    city VARCHAR(100) NOT NULL,
    landmark VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    zipcode VARCHAR(20) NOT NULL,
    country VARCHAR(100) NOT NULL,
    country_code VARCHAR(10) NOT NULL,
    latitude DECIMAL(10, 8) NULL,
    longitude DECIMAL(11, 8) NULL,
    business_license TEXT NOT NULL,
    articles_of_incorporation TEXT NOT NULL,
    national_identity_card TEXT NOT NULL,
    authorized_signature TEXT NOT NULL,
    verification_status VARCHAR(20) NOT NULL,
    metadata JSON NOT NULL,
    visibility_status VARCHAR(20) NOT NULL,
    deleted_at TIMESTAMP NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Stores
CREATE TABLE IF NOT EXISTS stores (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    seller_id BIGINT UNSIGNED NOT NULL,
    delivery_zone_id BIGINT UNSIGNED NULL,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(300) NOT NULL,
    address VARCHAR(255) NOT NULL,
    city VARCHAR(100) NOT NULL,
    landmark VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,
    zipcode VARCHAR(20) NOT NULL,
    country VARCHAR(100) NOT NULL,
    country_code VARCHAR(10) NOT NULL,
    latitude DECIMAL(10, 8) NOT NULL,
    longitude DECIMAL(11, 8) NOT NULL,
    contact_email VARCHAR(50) NOT NULL,
    contact_number VARCHAR(20) NOT NULL,
    description TEXT NULL,
    store_url VARCHAR(255) NULL,
    timing VARCHAR(500) NULL,
    address_proof TEXT NOT NULL,
    voided_check TEXT NOT NULL,
    tax_name VARCHAR(250) NOT NULL,
    tax_number VARCHAR(250) NOT NULL,
    bank_name VARCHAR(250) NOT NULL,
    bank_branch_code VARCHAR(250) NOT NULL,
    account_holder_name VARCHAR(250) NOT NULL,
    account_number VARCHAR(250) NOT NULL,
    routing_number VARCHAR(250) NOT NULL,
    bank_account_type VARCHAR(20) NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    permissions TEXT NULL,
    time_slot_config JSON NULL,
    max_delivery_distance DOUBLE NOT NULL,
    shipping_min_free_delivery_amount DOUBLE NOT NULL,
    shipping_charge_priority VARCHAR(255) NULL,
    allowed_order_per_time_slot INT NULL,
    order_preparation_time INT NOT NULL,
    carrier_partner TEXT NULL,
    promotional_text VARCHAR(1024) NULL,
    about_us TEXT NOT NULL,
    return_replacement_policy TEXT NULL,
    refund_policy TEXT NULL,
    terms_and_conditions TEXT NULL,
    delivery_policy TEXT NULL,
    shipping_preference TEXT NULL,
    domestic_shipping_charges DECIMAL(10, 2) NULL,
    international_shipping_charges DECIMAL(10, 2) NULL,
    metadata JSON NOT NULL,
    verification_status VARCHAR(20) NOT NULL,
    visibility_status VARCHAR(20) NOT NULL,
    fulfillment_type VARCHAR(20) NOT NULL,
    status VARCHAR(20) NULL,
    deleted_at TIMESTAMP NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

