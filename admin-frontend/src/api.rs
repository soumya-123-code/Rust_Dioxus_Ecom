use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

const API_BASE: &str = "http://localhost:3000/api/admin";

pub async fn get<T: DeserializeOwned>(endpoint: &str) -> Result<T, String> {
    let token: Option<String> = LocalStorage::get("admin_token").ok();
    
    let mut req = Request::get(&format!("{}{}", API_BASE, endpoint));
    
    if let Some(token) = token {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
    
    let response = req
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.ok() {
        response.json::<T>().await.map_err(|e| e.to_string())
    } else {
        let text = response.text().await.unwrap_or_default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message").and_then(|m| m.as_str()) {
                return Err(msg.to_string());
            }
        }
        Err(format!("Request failed: {} - {}", response.status(), text))
    }
}

pub async fn post<T: DeserializeOwned, B: Serialize>(endpoint: &str, body: &B) -> Result<T, String> {
    let token: Option<String> = LocalStorage::get("admin_token").ok();
    
    let mut req = Request::post(&format!("{}{}", API_BASE, endpoint))
        .header("Content-Type", "application/json");
    
    if let Some(token) = token {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }

    let req = req.json(body)
        .map_err(|e| e.to_string())?;
    
    let response = req
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.ok() {
        response.json::<T>().await.map_err(|e| e.to_string())
    } else {
        let text = response.text().await.unwrap_or_default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message").and_then(|m| m.as_str()) {
                return Err(msg.to_string());
            }
        }
        Err(format!("Request failed: {} - {}", response.status(), text))
    }
}

pub async fn put<T: DeserializeOwned, B: Serialize>(endpoint: &str, body: &B) -> Result<T, String> {
    let token: Option<String> = LocalStorage::get("admin_token").ok();
    
    let mut req = Request::put(&format!("{}{}", API_BASE, endpoint))
        .header("Content-Type", "application/json");
    
    if let Some(token) = token {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }

    let req = req.json(body)
        .map_err(|e| e.to_string())?;
    
    let response = req
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.ok() {
        response.json::<T>().await.map_err(|e| e.to_string())
    } else {
        let text = response.text().await.unwrap_or_default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message").and_then(|m| m.as_str()) {
                return Err(msg.to_string());
            }
        }
        Err(format!("Request failed: {} - {}", response.status(), text))
    }
}

pub async fn patch<T: DeserializeOwned, B: Serialize>(endpoint: &str, body: &B) -> Result<T, String> {
    let token: Option<String> = LocalStorage::get("admin_token").ok();
    
    // Workaround if Request::new is not found: use post and override method
    let mut req = Request::post(&format!("{}{}", API_BASE, endpoint))
        .method(gloo_net::http::Method::PATCH)
        .header("Content-Type", "application/json");
    
    if let Some(token) = token {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }

    let req = req.json(body)
        .map_err(|e| e.to_string())?;
    
    let response = req
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.ok() {
        response.json::<T>().await.map_err(|e| e.to_string())
    } else {
        let text = response.text().await.unwrap_or_default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message").and_then(|m| m.as_str()) {
                return Err(msg.to_string());
            }
        }
        Err(format!("Request failed: {} - {}", response.status(), text))
    }
}

pub async fn delete(endpoint: &str) -> Result<(), String> {
    let token: Option<String> = LocalStorage::get("admin_token").ok();
    
    let mut req = Request::delete(&format!("{}{}", API_BASE, endpoint));
    
    if let Some(token) = token {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
    
    let response = req
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.ok() {
        Ok(())
    } else {
        let text = response.text().await.unwrap_or_default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message").and_then(|m| m.as_str()) {
                return Err(msg.to_string());
            }
        }
        Err(format!("Request failed: {} - {}", response.status(), text))
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

impl<T> Default for PaginatedResponse<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            total: 0,
            page: 1,
            per_page: 10,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: super::state::User,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DashboardStats {
    pub total_orders: i64,
    pub total_products: i64,
    pub total_sellers: i64,
    pub total_customers: i64,
    pub total_revenue: String,
    pub pending_orders: i64,
    pub delivered_orders: i64,
    pub pending_seller_approvals: i64,
}
