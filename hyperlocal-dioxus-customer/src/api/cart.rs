use crate::types::cart::{AddToCartRequest, Cart};
use crate::api::client::ApiClient;
use serde_json::{json, Value};

#[derive(Clone)]
pub struct CartApi {
    api: ApiClient,
}

impl CartApi {
    pub fn new(base_url: String) -> Self {
        Self {
            api: ApiClient::new(base_url),
        }
    }

    pub async fn get_cart(&self, token: &str) -> Result<Cart, String> {
        match self.api.get_with_auth("user/cart", token).await {
            Ok(response) => {
                serde_json::from_value(response)
                    .map_err(|e| format!("Failed to parse cart: {}", e))
            }
            Err(e) => Err(format!("Failed to fetch cart: {}", e)),
        }
    }

    pub async fn add_to_cart(
        &self,
        request: AddToCartRequest,
        token: &str,
    ) -> Result<Value, String> {
        let body = json!({
            "product_id": request.product_id,
            "quantity": request.quantity,
            "variant_id": request.variant_id,
        });

        self.api
            .post_with_auth("user/cart/add", body, token)
            .await
            .map_err(|e| format!("Failed to add to cart: {}", e))
    }

    pub async fn remove_from_cart(&self, item_id: i32, token: &str) -> Result<Value, String> {
        let endpoint = format!("user/cart/item/{}", item_id);
        self.api
            .delete_with_auth(&endpoint, token)
            .await
            .map_err(|e| format!("Failed to remove from cart: {}", e))
    }

    pub async fn update_quantity(
        &self,
        item_id: i32,
        quantity: i32,
        token: &str,
    ) -> Result<Value, String> {
        let endpoint = format!("user/cart/item/{}", item_id);
        let body = json!({
            "quantity": quantity,
        });

        self.api
            .put_with_auth(&endpoint, body, token)
            .await
            .map_err(|e| format!("Failed to update quantity: {}", e))
    }

    pub async fn clear_cart(&self, token: &str) -> Result<Value, String> {
        self.api
            .post_with_auth("user/cart/clear-cart", json!({}), token)
            .await
            .map_err(|e| format!("Failed to clear cart: {}", e))
    }
}
