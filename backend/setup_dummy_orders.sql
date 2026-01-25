SET FOREIGN_KEY_CHECKS=0;
TRUNCATE TABLE orders;
TRUNCATE TABLE order_items;
TRUNCATE TABLE seller_orders;
SET FOREIGN_KEY_CHECKS=1;
INSERT IGNORE INTO users (id, name, email, password, created_at, updated_at) VALUES (101, 'Demo Customer', 'customer@example.com', '$2y$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', '2026-01-25 00:00:41', '2026-01-25 00:00:41');
INSERT IGNORE INTO model_has_roles (role_id, model_type, model_id) VALUES (4, 'App\\Models\\User', 101);

INSERT IGNORE INTO sellers (
    id, user_id, address, city, landmark, state, zipcode, country, country_code, 
    business_license, articles_of_incorporation, national_identity_card, authorized_signature, 
    verification_status, metadata, visibility_status, created_at, updated_at
) VALUES (
    100, 100, '123 Seller St', 'Seller City', 'Near Landmark', 'Seller State', '12345', 'CountryName', 'US',
    'license_doc', 'articles_doc', 'id_card_doc', 'signature_doc',
    'approved', '{"key": "value"}', 'visible', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
);


INSERT IGNORE INTO stores (
    id, seller_id, name, slug, address, city, landmark, state, zipcode, country, country_code,
    latitude, longitude, contact_email, contact_number, metadata, verification_status, visibility_status, fulfillment_type,
    created_at, updated_at, max_delivery_distance, shipping_min_free_delivery_amount, order_preparation_time, about_us
) VALUES (
    1, 100, 'Demo Store', 'demo-store', '123 Store St', 'Store City', 'Near Plaza', 'Store State', '54321', 'CountryName', 'US',
    0.0, 0.0, 'store@example.com', '1234567890', '{"key": "value"}', 'approved', 'visible', 'delivery',
    '2026-01-25 00:00:41', '2026-01-25 00:00:41', 100, 50, 30, 'About Us Text'
);


    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        1, '4fbe77e2-397e-4164-bd0a-0671dbf06642', 101, 268, 278, 278, 'cancelled',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        1, 100, 1, 268, 0.00, 268, 'cancelled', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            1, 13, 0, 1,
            'Product 13', 'Default', 'SKU-13',
            72, 2, 144,
            'cancelled', '0',
            0, 0, 0, 0, 72,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            1, 15, 0, 1,
            'Product 15', 'Default', 'SKU-15',
            29, 3, 87,
            'cancelled', '0',
            0, 0, 0, 0, 29,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            1, 2, 0, 1,
            'Product 2', 'Default', 'SKU-2',
            56, 1, 56,
            'cancelled', '0',
            0, 0, 0, 0, 56,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            1, 19, 0, 1,
            'Product 19', 'Default', 'SKU-19',
            88, 2, 176,
            'cancelled', '0',
            0, 0, 0, 0, 88,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        2, '63e7db91-1cc0-4718-8d38-da8846a3a1cc', 101, 304, 314, 314, 'pending',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        2, 100, 1, 304, 0.00, 304, 'pending', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            2, 14, 0, 1,
            'Product 14', 'Default', 'SKU-14',
            16, 2, 32,
            'pending', '0',
            0, 0, 0, 0, 16,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            2, 4, 0, 1,
            'Product 4', 'Default', 'SKU-4',
            82, 1, 82,
            'pending', '0',
            0, 0, 0, 0, 82,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            2, 12, 0, 1,
            'Product 12', 'Default', 'SKU-12',
            68, 1, 68,
            'pending', '0',
            0, 0, 0, 0, 68,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            2, 5, 0, 1,
            'Product 5', 'Default', 'SKU-5',
            10, 1, 10,
            'pending', '0',
            0, 0, 0, 0, 10,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            2, 16, 0, 1,
            'Product 16', 'Default', 'SKU-16',
            52, 1, 52,
            'pending', '0',
            0, 0, 0, 0, 52,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        3, '3a069f8e-89d0-4776-bcf6-ceb3f41fb76c', 101, 500, 510, 510, 'pending',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        3, 100, 1, 500, 0.00, 500, 'pending', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            3, 13, 0, 1,
            'Product 13', 'Default', 'SKU-13',
            59, 3, 177,
            'pending', '0',
            0, 0, 0, 0, 59,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            3, 18, 0, 1,
            'Product 18', 'Default', 'SKU-18',
            30, 3, 90,
            'pending', '0',
            0, 0, 0, 0, 30,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            3, 14, 0, 1,
            'Product 14', 'Default', 'SKU-14',
            19, 2, 38,
            'pending', '0',
            0, 0, 0, 0, 19,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            3, 10, 0, 1,
            'Product 10', 'Default', 'SKU-10',
            13, 1, 13,
            'pending', '0',
            0, 0, 0, 0, 13,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        4, 'aef236f1-b63c-41af-9b3a-fad5370929bf', 101, 52, 62, 62, 'pending',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        4, 100, 1, 52, 0.00, 52, 'pending', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            4, 8, 0, 1,
            'Product 8', 'Default', 'SKU-8',
            58, 1, 58,
            'pending', '0',
            0, 0, 0, 0, 58,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            4, 14, 0, 1,
            'Product 14', 'Default', 'SKU-14',
            83, 3, 249,
            'pending', '0',
            0, 0, 0, 0, 83,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        5, 'de6af381-73a9-439a-9692-664e8ed5cf0d', 101, 454, 464, 464, 'completed',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        5, 100, 1, 454, 0.00, 454, 'completed', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            5, 18, 0, 1,
            'Product 18', 'Default', 'SKU-18',
            45, 2, 90,
            'completed', '0',
            0, 0, 0, 0, 45,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            5, 5, 0, 1,
            'Product 5', 'Default', 'SKU-5',
            52, 2, 104,
            'completed', '0',
            0, 0, 0, 0, 52,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        6, '4adb15d8-7d6c-4b07-b4d3-7279592cec98', 101, 359, 369, 369, 'completed',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        6, 100, 1, 359, 0.00, 359, 'completed', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            6, 8, 0, 1,
            'Product 8', 'Default', 'SKU-8',
            35, 3, 105,
            'completed', '0',
            0, 0, 0, 0, 35,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            6, 12, 0, 1,
            'Product 12', 'Default', 'SKU-12',
            98, 2, 196,
            'completed', '0',
            0, 0, 0, 0, 98,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            6, 6, 0, 1,
            'Product 6', 'Default', 'SKU-6',
            83, 3, 249,
            'completed', '0',
            0, 0, 0, 0, 83,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        7, 'e7c38ad0-3f08-482b-9649-cf8d90531dc6', 101, 476, 486, 486, 'processing',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        7, 100, 1, 476, 0.00, 476, 'processing', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            7, 11, 0, 1,
            'Product 11', 'Default', 'SKU-11',
            44, 3, 132,
            'processing', '0',
            0, 0, 0, 0, 44,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        8, 'a57bbeef-161a-407a-bb1b-0a47dcb1158e', 101, 451, 461, 461, 'completed',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        8, 100, 1, 451, 0.00, 451, 'completed', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            8, 16, 0, 1,
            'Product 16', 'Default', 'SKU-16',
            85, 2, 170,
            'completed', '0',
            0, 0, 0, 0, 85,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        9, '4d4aff48-ac52-46ef-972b-0c78ac52a470', 101, 295, 305, 305, 'pending',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        9, 100, 1, 295, 0.00, 295, 'pending', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            9, 3, 0, 1,
            'Product 3', 'Default', 'SKU-3',
            80, 2, 160,
            'pending', '0',
            0, 0, 0, 0, 80,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        10, 'c945adec-b671-4631-ae7f-5238769cf33d', 101, 357, 367, 367, 'pending',
        'Demo Customer', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US',
        'Demo Customer', '123 Shipping St', NULL, 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US', 
        '2026-01-25 00:00:41', '2026-01-25 00:00:41', 10.00
    );
    

    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        10, 100, 1, 357, 0.00, 357, 'pending', '2026-01-25 00:00:41', '2026-01-25 00:00:41'
    );
    

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            10, 20, 0, 1,
            'Product 20', 'Default', 'SKU-20',
            30, 2, 60,
            'pending', '0',
            0, 0, 0, 0, 30,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            10, 14, 0, 1,
            'Product 14', 'Default', 'SKU-14',
            17, 1, 17,
            'pending', '0',
            0, 0, 0, 0, 17,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            10, 11, 0, 1,
            'Product 11', 'Default', 'SKU-11',
            65, 1, 65,
            'pending', '0',
            0, 0, 0, 0, 65,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            10, 7, 0, 1,
            'Product 7', 'Default', 'SKU-7',
            11, 3, 33,
            'pending', '0',
            0, 0, 0, 0, 11,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        

        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            10, 2, 0, 1,
            'Product 2', 'Default', 'SKU-2',
            48, 2, 96,
            'pending', '0',
            0, 0, 0, 0, 48,
            '2026-01-25 00:00:41', '2026-01-25 00:00:41'
        );
        
