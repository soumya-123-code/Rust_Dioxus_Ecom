use crate::api::client::ApiClient;
use crate::types::address::{Address, AddressRequest};
use serde_json::{json, Value};

#[derive(Clone)]
pub struct AddressesApi {
    api: ApiClient,
}

impl AddressesApi {
    pub fn new(base_url: String) -> Self {
        Self {
            api: ApiClient::new(base_url),
        }
    }

    pub async fn get_addresses(&self, token: &str) -> Result<Vec<Address>, String> {
        match self.api.get_with_auth("user/addresses", token).await {
            Ok(response) => {
                // Parse addresses from response
                Ok(vec![])
            }
            Err(e) => Err(format!("Failed to fetch addresses: {}", e)),
        }
    }

    pub async fn add_address(&self, address: AddressRequest, token: &str) -> Result<Value, String> {
        let body = serde_json::to_value(address)
            .map_err(|e| format!("Failed to serialize address: {}", e))?;
        
        self.api
            .post_with_auth("user/addresses", body, token)
            .await
            .map_err(|e| format!("Failed to add address: {}", e))
    }

    pub async fn update_address(
        &self,
        address_id: i32,
        address: AddressRequest,
        token: &str,
    ) -> Result<Value, String> {
        let endpoint = format!("user/addresses/{}", address_id);
        let body = serde_json::to_value(address)
            .map_err(|e| format!("Failed to serialize address: {}", e))?;
        
        self.api
            .put_with_auth(&endpoint, body, token)
            .await
            .map_err(|e| format!("Failed to update address: {}", e))
    }

    pub async fn delete_address(&self, address_id: i32, token: &str) -> Result<Value, String> {
        let endpoint = format!("user/addresses/{}", address_id);
        self.api
            .delete_with_auth(&endpoint, token)
            .await
            .map_err(|e| format!("Failed to delete address: {}", e))
    }
}
