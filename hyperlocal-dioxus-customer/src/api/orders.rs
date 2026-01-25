use crate::api::client::ApiClient;
use serde_json::Value;

#[derive(Clone)]
pub struct OrdersApi {
    api: ApiClient,
}

impl OrdersApi {
    pub fn new(base_url: String) -> Self {
        Self {
            api: ApiClient::new(base_url),
        }
    }

    pub async fn get_orders(&self, token: &str) -> Result<Value, String> {
        self.api
            .get_with_auth("user/orders", token)
            .await
            .map_err(|e| format!("Failed to fetch orders: {}", e))
    }

    pub async fn get_order_detail(&self, order_slug: &str, token: &str) -> Result<Value, String> {
        let endpoint = format!("user/orders/{}", order_slug);
        self.api
            .get_with_auth(&endpoint, token)
            .await
            .map_err(|e| format!("Failed to fetch order: {}", e))
    }

    pub async fn create_order(&self, order_data: Value, token: &str) -> Result<Value, String> {
        self.api
            .post_with_auth("user/orders", order_data, token)
            .await
            .map_err(|e| format!("Failed to create order: {}", e))
    }
}
