import uuid
import random
import datetime

def get_timestamp():
    return datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')

sql_statements = []

sql_statements.append("SET FOREIGN_KEY_CHECKS=0;")
sql_statements.append("TRUNCATE TABLE orders;")
sql_statements.append("TRUNCATE TABLE order_items;")
sql_statements.append("TRUNCATE TABLE seller_orders;")
sql_statements.append("SET FOREIGN_KEY_CHECKS=1;")

# 1. Create Customer User
customer_id = 101
customer_email = 'customer@example.com'
customer_name = 'Demo Customer'
password_hash = '$2y$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi' 

sql_statements.append(f"INSERT IGNORE INTO users (id, name, email, password, created_at, updated_at) VALUES ({customer_id}, '{customer_name}', '{customer_email}', '{password_hash}', '{get_timestamp()}', '{get_timestamp()}');")
# Assign customer role (id 4)
sql_statements.append(f"INSERT IGNORE INTO model_has_roles (role_id, model_type, model_id) VALUES (4, 'App\\\\Models\\\\User', {customer_id});")

# 2. Create Seller Entry (id=100 to match products)
seller_id = 100
user_id = 100 # The user we created in previous script
# Seller fields: address, city, landmark, state, zipcode, country, country_code, business_license, articles_of_incorporation, national_identity_card, authorized_signature, verification_status, metadata, visibility_status
sql_statements.append(f"""
INSERT IGNORE INTO sellers (
    id, user_id, address, city, landmark, state, zipcode, country, country_code, 
    business_license, articles_of_incorporation, national_identity_card, authorized_signature, 
    verification_status, metadata, visibility_status, created_at, updated_at
) VALUES (
    {seller_id}, {user_id}, '123 Seller St', 'Seller City', 'Near Landmark', 'Seller State', '12345', 'CountryName', 'US',
    'license_doc', 'articles_doc', 'id_card_doc', 'signature_doc',
    'approved', '{{"key": "value"}}', 'visible', '{get_timestamp()}', '{get_timestamp()}'
);
""")

# 3. Create Store Entry
store_id = 1
store_name = "Demo Store"
store_slug = "demo-store"
# Store fields: seller_id, name, slug, address, city, landmark, state, zipcode, country, country_code, latitude, longitude, contact_email, contact_number, metadata, verification_status, visibility_status, fulfillment_type
sql_statements.append(f"""
INSERT IGNORE INTO stores (
    id, seller_id, name, slug, address, city, landmark, state, zipcode, country, country_code,
    latitude, longitude, contact_email, contact_number, metadata, verification_status, visibility_status, fulfillment_type,
    created_at, updated_at, max_delivery_distance, shipping_min_free_delivery_amount, order_preparation_time, about_us
) VALUES (
    {store_id}, {seller_id}, '{store_name}', '{store_slug}', '123 Store St', 'Store City', 'Near Plaza', 'Store State', '54321', 'CountryName', 'US',
    0.0, 0.0, 'store@example.com', '1234567890', '{{"key": "value"}}', 'approved', 'visible', 'delivery',
    '{get_timestamp()}', '{get_timestamp()}', 100, 50, 30, 'About Us Text'
);
""")

# 4. Create Orders and Order Items
statuses = ['pending', 'processing', 'completed', 'cancelled']
products = list(range(1, 21)) # Assuming products 1-20 exist

for i in range(1, 11): # 10 orders
    order_id = i
    order_uuid = str(uuid.uuid4())
    status = random.choice(statuses)
    subtotal = random.randint(50, 500)
    total = subtotal + 10 # shipping
    
    # Order fields: uuid, user_id, order_number(not in describe?), subtotal, total_payable, final_total, status, billing_*, shipping_*
    # I'll check describe again if I miss anything. Based on previous describe:
    # billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code
    # shipping_name... same
    
    billing_info = f"'{customer_name}', '123 Billing St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'BillCity', 'BillState', 'BillCountry', 'US'"
    shipping_info = f"'{customer_name}', '123 Shipping St', 'Landmark', '11111', '9999999999', 'home', 0.0, 0.0, 'ShipCity', 'ShipState', 'ShipCountry', 'US'"
    
    sql_statements.append(f"""
    INSERT IGNORE INTO orders (
        id, uuid, user_id, subtotal, total_payable, final_total, status,
        billing_name, billing_address_1, billing_landmark, billing_zip, billing_phone, billing_address_type, billing_latitude, billing_longitude, billing_city, billing_state, billing_country, billing_country_code,
        shipping_name, shipping_address_1, shipping_address_2, shipping_landmark, shipping_zip, shipping_phone, shipping_address_type, shipping_latitude, shipping_longitude, shipping_city, shipping_state, shipping_country, shipping_country_code,
        created_at, updated_at, delivery_charge
    ) VALUES (
        {order_id}, '{order_uuid}', {customer_id}, {subtotal}, {total}, {total}, '{status}',
        {billing_info},
        {shipping_info.replace("'123 Shipping St'", "'123 Shipping St', NULL")}, 
        '{get_timestamp()}', '{get_timestamp()}', 10.00
    );
    """)

    # Seller Order
    sql_statements.append(f"""
    INSERT INTO seller_orders (
        order_id, seller_id, store_id, subtotal, admin_commission, seller_earning, status, created_at, updated_at
    ) VALUES (
        {order_id}, {seller_id}, {store_id}, {subtotal}, 0.00, {subtotal}, '{status}', '{get_timestamp()}', '{get_timestamp()}'
    );
    """)
    
    # Order Items
    num_items = random.randint(1, 5)
    selected_products = random.sample(products, num_items)
    
    for prod_id in selected_products:
        item_price = random.randint(10, 100)
        qty = random.randint(1, 3)
        item_subtotal = item_price * qty
        
        # OrderItem fields: order_id, product_id, product_variant_id, store_id, title, variant_title, price, quantity, subtotal, status, commission_settled
        # Missing: sku, tax_amount etc.
        # SKU is required (NO NULL).
        
        sql_statements.append(f"""
        INSERT IGNORE INTO order_items (
            order_id, product_id, product_variant_id, store_id, 
            title, variant_title, sku, 
            price, quantity, subtotal, 
            status, commission_settled, 
            gift_card_discount, admin_commission_amount, seller_commission_amount, discount, discounted_price,
            created_at, updated_at
        ) VALUES (
            {order_id}, {prod_id}, 0, {store_id},
            'Product {prod_id}', 'Default', 'SKU-{prod_id}',
            {item_price}, {qty}, {item_subtotal},
            '{status}', '0',
            0, 0, 0, 0, {item_price},
            '{get_timestamp()}', '{get_timestamp()}'
        );
        """)

with open('setup_dummy_orders.sql', 'w') as f:
    for stmt in sql_statements:
        f.write(stmt + "\n")

print("SQL file 'setup_dummy_orders.sql' created.")
