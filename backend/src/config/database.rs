use diesel::r2d2::{self, ConnectionManager};
use diesel::MysqlConnection;
use std::env;

use crate::utils::types::DbPool;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    match r2d2::Pool::builder()
        .max_size(10)
        .build(manager) {
            Ok(pool) => {
                println!("Database connection established successfully.");
                pool
            },
            Err(e) => {
                println!("Failed to establish database connection: {}", e);
                panic!("Failed to create database pool: {}", e);
            }
        }
}
