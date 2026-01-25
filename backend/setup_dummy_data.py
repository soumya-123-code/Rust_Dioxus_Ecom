import uuid
import random
import datetime

def get_timestamp():
    return datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')

def escape_sql(val):
    if val is None:
        return 'NULL'
    if isinstance(val, int) or isinstance(val, float):
        return str(val)
    return "'" + str(val).replace("'", "''") + "'"

sql_statements = []

# 1. Create a dummy seller user
seller_id = 100
seller_email = 'seller@example.com'
seller_name = 'Demo Seller'
# Password hash for 'password' (bcrypt) - just using a placeholder or a known hash if possible. 
# Since I can't generate bcrypt easily without a library, I'll use a dummy hash.
# Or better, just assume the user can reset it or doesn't need to login immediately with this specific account.
# But for completeness, I'll use a dummy string.
password_hash = '$2y$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi' # password
sql_statements.append(f"INSERT IGNORE INTO users (id, name, email, password, created_at, updated_at) VALUES ({seller_id}, '{seller_name}', '{seller_email}', '{password_hash}', '{get_timestamp()}', '{get_timestamp()}');")

# Assign seller role (id 3) to this user
sql_statements.append(f"INSERT IGNORE INTO model_has_roles (role_id, model_type, model_id) VALUES (3, 'App\\\\Models\\\\User', {seller_id});")

# 2. Create Categories
category_ids = []
for i in range(11, 21):
    cat_id = i
    category_ids.append(cat_id)
    cat_uuid = str(uuid.uuid4())
    title = f"Category {i}"
    slug = f"category-{i}"
    image = f"https://via.placeholder.com/300?text=Category+{i}"
    description = f"Description for Category {i}"
    metadata = '{"seo_title": "SEO Title", "seo_description": "SEO Description"}'
    
    sql = f"""
    INSERT IGNORE INTO categories (id, uuid, title, slug, image, description, status, requires_approval, metadata, created_at, updated_at)
    VALUES ({cat_id}, '{cat_uuid}', '{title}', '{slug}', '{image}', '{description}', 'active', 0, '{metadata}', '{get_timestamp()}', '{get_timestamp()}');
    """
    sql_statements.append(sql.strip())

# 3. Create Brands
brand_ids = []
for i in range(1, 6):
    brand_id = i
    brand_ids.append(brand_id)
    brand_uuid = str(uuid.uuid4())
    title = f"Brand {i}"
    slug = f"brand-{i}"
    image = f"https://via.placeholder.com/150?text=Brand+{i}"
    description = f"Description for Brand {i}"
    metadata = '{"website": "https://example.com"}'
    
    sql = f"""
    INSERT IGNORE INTO brands (id, uuid, slug, title, description, image, status, metadata, created_at, updated_at)
    VALUES ({brand_id}, '{brand_uuid}', '{slug}', '{title}', '{description}', '{image}', '1', '{metadata}', '{get_timestamp()}', '{get_timestamp()}');
    """
    sql_statements.append(sql.strip())

# 4. Create Product Conditions
condition_ids = []
conditions = ['New', 'Used', 'Refurbished']
for i, cond in enumerate(conditions):
    cond_id = i + 1
    condition_ids.append(cond_id)
    slug = cond.lower()
    
    sql = f"""
    INSERT IGNORE INTO product_conditions (id, title, slug, created_at, updated_at)
    VALUES ({cond_id}, '{cond}', '{slug}', '{get_timestamp()}', '{get_timestamp()}');
    """
    sql_statements.append(sql.strip())

# 5. Create Products
for i in range(1, 21):
    prod_uuid = str(uuid.uuid4())
    cat_id = random.choice(category_ids)
    brand_id = random.choice(brand_ids)
    cond_id = random.choice(condition_ids)
    title = f"Product {i}"
    slug = f"product-{i}"
    short_desc = f"Short description for Product {i}"
    desc = f"Long description for Product {i}. This is a dummy product generated for testing purposes."
    price = random.randint(10, 1000)
    qty = random.randint(10, 100)
    metadata = '{"key": "value"}'
    tags = "tag1, tag2"
    
    # Note: Using default values for many columns where possible
    sql = f"""
    INSERT INTO products (
        uuid, seller_id, category_id, brand_id, product_condition_id, 
        slug, title, type, short_description, description, 
        minimum_order_quantity, quantity_step_size, total_allowed_quantity, 
        is_inclusive_tax, is_returnable, is_cancelable, is_attachment_required, 
        status, featured, tags, metadata, created_at, updated_at
    ) VALUES (
        '{prod_uuid}', {seller_id}, {cat_id}, {brand_id}, {cond_id},
        '{slug}', '{title}', 'simple', '{short_desc}', '{desc}',
        1, 1, {qty},
        '0', '0', '0', '0',
        'active', '0', '{tags}', '{metadata}', '{get_timestamp()}', '{get_timestamp()}'
    );
    """
    sql_statements.append(sql.strip())

with open('setup_dummy_data.sql', 'w') as f:
    for stmt in sql_statements:
        f.write(stmt + "\n")

print("SQL file 'setup_dummy_data.sql' created.")
