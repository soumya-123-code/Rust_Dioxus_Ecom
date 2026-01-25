import re
import sys

def parse_php_value(val):
    val = val.strip()
    if val.startswith("'") and val.endswith("'"):
        # Remove outer quotes
        inner = val[1:-1]
        # Unescape \' to '
        inner = inner.replace("\\'", "'")
        # Escape ' to '' for SQL
        return "'" + inner.replace("'", "''") + "'"
    if val == 'NULL':
        return 'NULL'
    return val

def main():
    input_file = r"d:\projects\Rust_React_MultiVendor\6mart\hyperLocal-multivendor-eCommerce-backend-admin-seller-v1.0.0\database\seeders\CountriesSeeder.php"
    output_file = r"d:\projects\Rust_React_MultiVendor\6mart\rust_admin_project\backend\setup_countries.sql"

    with open(input_file, 'r', encoding='utf-8') as f:
        content = f.read()

    # Create table logic remains...
    create_table_sql = """CREATE TABLE IF NOT EXISTS countries (
    id BIGINT UNSIGNED PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    iso3 VARCHAR(3) NULL,
    numeric_code VARCHAR(3) NULL,
    iso2 VARCHAR(2) NULL,
    phonecode VARCHAR(255) NULL,
    capital VARCHAR(255) NULL,
    currency VARCHAR(255) NULL,
    currency_name VARCHAR(255) NULL,
    currency_symbol VARCHAR(255) NULL,
    tld VARCHAR(255) NULL,
    native VARCHAR(255) NULL,
    region VARCHAR(255) NULL,
    subregion VARCHAR(255) NULL,
    timezones TEXT NULL,
    translations TEXT NULL,
    latitude DECIMAL(10, 8) NULL,
    longitude DECIMAL(11, 8) NULL,
    emoji VARCHAR(191) NULL,
    emojiU VARCHAR(191) NULL,
    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    flag BOOLEAN NOT NULL DEFAULT 1,
    wikiDataId VARCHAR(255) NULL
);
"""

    # Extract the block containing the arrays
    start_marker = "$countries = array("
    # The end marker might vary, let's just take everything after start
    start_idx = content.find(start_marker)
    if start_idx == -1:
        print("Could not find start marker")
        return

    content_subset = content[start_idx:]
    
    # regex to match each array(...) item
    # array('id' => '...', ...),
    # Use dotall and non-greedy match for the array content
    pattern = re.compile(r"array\((.*?)\),", re.DOTALL)
    
    matches = pattern.findall(content_subset)
    print(f"Found {len(matches)} countries")
    
    sql_inserts = []
    
    # Regex to capture key-value pairs
    # key is 'word'
    # value is either 'string' (handling escapes) or NULL or number (but here mostly strings)
    # The string pattern: ' ( [^'\\]* (?: \\. [^'\\]* )* ) '
    kv_pattern = re.compile(r"'(\w+)'\s*=>\s*('(?:\\[\s\S]|[^'])*'|NULL|\d+)")
    
    all_rows_values = []
    cols = ['id', 'name', 'iso3', 'numeric_code', 'iso2', 'phonecode', 'capital', 
            'currency', 'currency_name', 'currency_symbol', 'tld', 'native', 
            'region', 'subregion', 'timezones', 'translations', 'latitude', 
            'longitude', 'emoji', 'emojiU', 'created_at', 'updated_at', 'flag', 'wikiDataId']

    for match in matches:
        kvs = kv_pattern.findall(match)
        
        data = {}
        for k, v in kvs:
            data[k] = parse_php_value(v)
            
        values = []
        for col in cols:
            values.append(data.get(col, 'NULL'))
            
        values_str = "(" + ", ".join(values) + ")"
        all_rows_values.append(values_str)

    if all_rows_values:
        sql_inserts.append(f"INSERT INTO countries ({', '.join(cols)}) VALUES\n" + ",\n".join(all_rows_values) + ";")

    # Add roles
    roles_sql = """
INSERT IGNORE INTO roles (name, guard_name, created_at, updated_at) VALUES 
('super_admin', 'admin', NOW(), NOW()),
('seller', 'seller', NOW(), NOW()),
('customer', 'web', NOW(), NOW());
"""

    with open(output_file, 'w', encoding='utf-8') as f:
        f.write(create_table_sql)
        f.write("\n")
        f.write("\n".join(sql_inserts))
        f.write("\n")
        f.write(roles_sql)

if __name__ == "__main__":
    main()
