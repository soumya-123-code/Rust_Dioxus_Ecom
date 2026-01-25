use diesel::prelude::*;
use crate::models::{User, UserResponse};
use crate::schema::users;
use crate::utils::{jwt, password};
use diesel::MysqlConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub struct AuthService;

impl AuthService {
    pub fn login(
        conn: &mut PooledConnection<ConnectionManager<MysqlConnection>>,
        email: &str,
        pass: &str,
        jwt_secret: &str,
    ) -> Result<(String, UserResponse), String> {
        let user: Option<User> = users::table
            .filter(users::email.eq(email))
            .filter(users::access_panel.eq("admin"))
            .select(User::as_select())
            .first(conn)
            .optional()
            .map_err(|e| e.to_string())?;

        let user = user.ok_or("Invalid credentials")?;

        if !password::verify(pass, &user.password) {
            return Err("Invalid credentials".to_string());
        }

        let token = jwt::generate_token(jwt_secret, user.id)
            .map_err(|e| e.to_string())?;

        Ok((token, UserResponse::from(user)))
    }
}
