INSERT IGNORE INTO users (id, name, email, password, created_at, updated_at) VALUES (100, 'Demo Seller', 'seller@example.com', '$2y$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO model_has_roles (role_id, model_type, model_id) VALUES (3, 'App\\Models\\User', 100);
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (11, '3277bbb4-aea7-4ea5-9e39-7436191df80a', 'Category 11', 'category-11', 'https://via.placeholder.com/300?text=Category+11', 'Description for Category 11', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (12, '0e8774c7-c28d-4126-8600-86b0d3614713', 'Category 12', 'category-12', 'https://via.placeholder.com/300?text=Category+12', 'Description for Category 12', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (13, 'f62d088e-78e1-467d-91fc-fd6ebe2c121e', 'Category 13', 'category-13', 'https://via.placeholder.com/300?text=Category+13', 'Description for Category 13', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (14, 'f3618583-f8fc-4138-bad4-2d85e79472f7', 'Category 14', 'category-14', 'https://via.placeholder.com/300?text=Category+14', 'Description for Category 14', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (15, '349e3588-066e-49b1-9d8b-e2ad1ab88165', 'Category 15', 'category-15', 'https://via.placeholder.com/300?text=Category+15', 'Description for Category 15', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (16, '2e077295-0c1d-4d07-9500-24f4f982c801', 'Category 16', 'category-16', 'https://via.placeholder.com/300?text=Category+16', 'Description for Category 16', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (17, '4ede1dc1-b243-44d3-97f0-b8c1a77fbac3', 'Category 17', 'category-17', 'https://via.placeholder.com/300?text=Category+17', 'Description for Category 17', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (18, '63181ba0-018e-41b4-94f7-e913d3771c3e', 'Category 18', 'category-18', 'https://via.placeholder.com/300?text=Category+18', 'Description for Category 18', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (19, 'c92fff74-3333-4eee-8cfe-61ca0b1c20d8', 'Category 19', 'category-19', 'https://via.placeholder.com/300?text=Category+19', 'Description for Category 19', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES (20, '5092950c-1568-41c6-9172-6548f8d4fae8', 'Category 20', 'category-20', 'https://via.placeholder.com/300?text=Category+20', 'Description for Category 20', 'active', 0, '{"seo_title": "SEO Title", "seo_description": "SEO Description"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO brands (id, uuid, slug, title, description, image, status, metadata, created_at, updated_at)
    VALUES (1, 'e79de703-87ba-4d77-a837-51125f79edee', 'brand-1', 'Brand 1', 'Description for Brand 1', 'https://via.placeholder.com/150?text=Brand+1', '1', '{"website": "https://example.com"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO brands (id, uuid, slug, title, description, image, status, metadata, created_at, updated_at)
    VALUES (2, '5047e2cc-e404-473b-8251-8b3780f02b32', 'brand-2', 'Brand 2', 'Description for Brand 2', 'https://via.placeholder.com/150?text=Brand+2', '1', '{"website": "https://example.com"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO brands (id, uuid, slug, title, description, image, status, metadata, created_at, updated_at)
    VALUES (3, 'a15ae0ca-75ce-462e-a723-be8bc3fb58b0', 'brand-3', 'Brand 3', 'Description for Brand 3', 'https://via.placeholder.com/150?text=Brand+3', '1', '{"website": "https://example.com"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO brands (id, uuid, slug, title, description, image, status, metadata, created_at, updated_at)
    VALUES (4, 'a93e16f4-2093-4b01-8ac3-a0dcc008a2c2', 'brand-4', 'Brand 4', 'Description for Brand 4', 'https://via.placeholder.com/150?text=Brand+4', '1', '{"website": "https://example.com"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO brands (id, uuid, slug, title, description, image, status, metadata, created_at, updated_at)
    VALUES (5, '93470157-7d0c-4dc4-913d-be84a101287e', 'brand-5', 'Brand 5', 'Description for Brand 5', 'https://via.placeholder.com/150?text=Brand+5', '1', '{"website": "https://example.com"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO product_conditions (id, title, slug, created_at, updated_at)
    VALUES (1, 'New', 'new', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO product_conditions (id, title, slug, created_at, updated_at)
    VALUES (2, 'Used', 'used', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT IGNORE INTO product_conditions (id, title, slug, created_at, updated_at)
    VALUES (3, 'Refurbished', 'refurbished', '2026-01-24 23:53:21', '2026-01-24 23:53:21');
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '2a6a748e-5e28-4168-b512-62c4e0e3fa12', 100, 16, 4, 2,
        'product-1', 'Product 1', 'simple', 'Short description for Product 1', 'Long description for Product 1. This is a dummy product generated for testing purposes.',
        1, 1, 96,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '5e5b7432-9f7a-4ea5-a155-c806797290f6', 100, 17, 1, 3,
        'product-2', 'Product 2', 'simple', 'Short description for Product 2', 'Long description for Product 2. This is a dummy product generated for testing purposes.',
        1, 1, 72,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        'd06df631-a2c4-4b40-9075-6bb094474ef3', 100, 19, 2, 2,
        'product-3', 'Product 3', 'simple', 'Short description for Product 3', 'Long description for Product 3. This is a dummy product generated for testing purposes.',
        1, 1, 66,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '047b8471-f1dd-4f08-9b44-e1e9bbb97202', 100, 18, 2, 2,
        'product-4', 'Product 4', 'simple', 'Short description for Product 4', 'Long description for Product 4. This is a dummy product generated for testing purposes.',
        1, 1, 11,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        'e3eb68a3-782e-4dfe-8a6e-c677a5008100', 100, 20, 1, 3,
        'product-5', 'Product 5', 'simple', 'Short description for Product 5', 'Long description for Product 5. This is a dummy product generated for testing purposes.',
        1, 1, 68,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '3ed60e30-03ad-4269-8312-d73b679241a8', 100, 13, 4, 2,
        'product-6', 'Product 6', 'simple', 'Short description for Product 6', 'Long description for Product 6. This is a dummy product generated for testing purposes.',
        1, 1, 33,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        'aea2733b-380e-4f6a-8a5c-59d0c0a7019d', 100, 12, 1, 2,
        'product-7', 'Product 7', 'simple', 'Short description for Product 7', 'Long description for Product 7. This is a dummy product generated for testing purposes.',
        1, 1, 20,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        'b235fb5f-a71c-4b42-b33b-079667933246', 100, 20, 3, 2,
        'product-8', 'Product 8', 'simple', 'Short description for Product 8', 'Long description for Product 8. This is a dummy product generated for testing purposes.',
        1, 1, 69,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '085b8d25-5c2d-4ae1-9941-4e688145104b', 100, 20, 1, 3,
        'product-9', 'Product 9', 'simple', 'Short description for Product 9', 'Long description for Product 9. This is a dummy product generated for testing purposes.',
        1, 1, 38,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '05f5c4e2-2521-4002-9cdc-cb88f6ca6963', 100, 16, 1, 3,
        'product-10', 'Product 10', 'simple', 'Short description for Product 10', 'Long description for Product 10. This is a dummy product generated for testing purposes.',
        1, 1, 44,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '7120c4e1-a749-498f-a68d-c86a922dd554', 100, 19, 2, 1,
        'product-11', 'Product 11', 'simple', 'Short description for Product 11', 'Long description for Product 11. This is a dummy product generated for testing purposes.',
        1, 1, 26,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        'd4da2e1c-b730-47a8-b67f-dae230058a46', 100, 13, 3, 3,
        'product-12', 'Product 12', 'simple', 'Short description for Product 12', 'Long description for Product 12. This is a dummy product generated for testing purposes.',
        1, 1, 100,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '348b98cb-883e-4ae0-8553-66d23ac927a2', 100, 17, 5, 1,
        'product-13', 'Product 13', 'simple', 'Short description for Product 13', 'Long description for Product 13. This is a dummy product generated for testing purposes.',
        1, 1, 64,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '6e6945b4-1c17-4358-8a7b-0745f8c7ae0c', 100, 13, 2, 2,
        'product-14', 'Product 14', 'simple', 'Short description for Product 14', 'Long description for Product 14. This is a dummy product generated for testing purposes.',
        1, 1, 43,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        'be0933ac-c7fe-48ff-af49-4240575ffc63', 100, 12, 5, 2,
        'product-15', 'Product 15', 'simple', 'Short description for Product 15', 'Long description for Product 15. This is a dummy product generated for testing purposes.',
        1, 1, 84,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '2cb5ee71-6ee2-40ef-914a-2f03cbb7325b', 100, 15, 4, 2,
        'product-16', 'Product 16', 'simple', 'Short description for Product 16', 'Long description for Product 16. This is a dummy product generated for testing purposes.',
        1, 1, 82,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '3872cf6d-035d-42dc-bda4-753a92286c30', 100, 11, 1, 3,
        'product-17', 'Product 17', 'simple', 'Short description for Product 17', 'Long description for Product 17. This is a dummy product generated for testing purposes.',
        1, 1, 65,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        'bd30d066-db84-43eb-a990-ee409be35c30', 100, 20, 5, 2,
        'product-18', 'Product 18', 'simple', 'Short description for Product 18', 'Long description for Product 18. This is a dummy product generated for testing purposes.',
        1, 1, 74,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '6091e870-d609-468e-97eb-4974bd6c09df', 100, 20, 3, 2,
        'product-19', 'Product 19', 'simple', 'Short description for Product 19', 'Long description for Product 19. This is a dummy product generated for testing purposes.',
        1, 1, 24,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '6de74136-0be6-4768-8d9e-a0139edd499f', 100, 13, 1, 1,
        'product-20', 'Product 20', 'simple', 'Short description for Product 20', 'Long description for Product 20. This is a dummy product generated for testing purposes.',
        1, 1, 96,
        '0', '0', '0', '0',
        'active', '0', 'tag1, tag2', '{"key": "value"}', '2026-01-24 23:53:21', '2026-01-24 23:53:21'
    );
