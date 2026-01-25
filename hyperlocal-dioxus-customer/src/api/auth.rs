use crate::types::{AuthModel, LoginRequest, RegisterRequest};
use crate::api::client::ApiClient;
use serde_json::{json, Value};

#[derive(Clone)]
pub struct AuthApi {
    api: ApiClient,
}

impl AuthApi {
    pub fn new(base_url: String) -> Self {
        Self {
            api: ApiClient::new(base_url),
        }
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthModel, String> {
        let body = json!({
            "email": request.email,
            "mobile": request.mobile,
            "password": request.password,
            "fcm_token": request.fcm_token,
            "device_type": request.device_type.unwrap_or_else(|| "web".to_string()),
        });

        match self.api.post("login", body).await {
            Ok(response) => {
                serde_json::from_value(response)
                    .map_err(|e| format!("Failed to parse response: {}", e))
            }
            Err(e) => Err(format!("Login failed: {}", e)),
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<AuthModel, String> {
        let body = json!({
            "name": request.name,
            "email": request.email,
            "mobile": request.mobile,
            "country": request.country,
            "iso2": request.iso2,
            "password": request.password,
            "password_confirmation": request.password_confirmation,
            "fcm_token": request.fcm_token,
            "device_type": request.device_type.unwrap_or_else(|| "web".to_string()),
        });

        match self.api.post("register", body).await {
            Ok(response) => {
                serde_json::from_value(response)
                    .map_err(|e| format!("Failed to parse response: {}", e))
            }
            Err(e) => Err(format!("Registration failed: {}", e)),
        }
    }

    pub async fn verify_user(&self, token: &str, verification_code: &str) -> Result<Value, String> {
        let body = json!({
            "verification_code": verification_code,
        });

        self.api
            .post_with_auth("verify-user", body, token)
            .await
            .map_err(|e| format!("Verification failed: {}", e))
    }

    pub async fn forgot_password(&self, email: &str) -> Result<Value, String> {
        let body = json!({
            "email": email,
        });

        self.api
            .post("forget-password", body)
            .await
            .map_err(|e| format!("Forgot password failed: {}", e))
    }
}
