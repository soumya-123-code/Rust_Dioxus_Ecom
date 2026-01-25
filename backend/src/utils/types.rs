use diesel::r2d2::{self, ConnectionManager};
use diesel::MysqlConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub jwt_secret: String,
}
