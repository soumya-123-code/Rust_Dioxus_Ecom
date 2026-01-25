use crate::api::client::ApiClient;
use serde_json::{json, Value};

#[derive(Clone)]
pub struct WalletApi {
    api: ApiClient,
}

impl WalletApi {
    pub fn new(base_url: String) -> Self {
        Self {
            api: ApiClient::new(base_url),
        }
    }

    pub async fn get_wallet(&self, token: &str) -> Result<Value, String> {
        self.api
            .get_with_auth("user/wallet", token)
            .await
            .map_err(|e| format!("Failed to fetch wallet: {}", e))
    }

    pub async fn get_transactions(&self, token: &str) -> Result<Value, String> {
        self.api
            .get_with_auth("user/wallet/transactions", token)
            .await
            .map_err(|e| format!("Failed to fetch transactions: {}", e))
    }

    pub async fn prepare_recharge(&self, amount: f64, token: &str) -> Result<Value, String> {
        let body = json!({
            "amount": amount,
        });

        self.api
            .post_with_auth("user/wallet/prepare-wallet-recharge", body, token)
            .await
            .map_err(|e| format!("Failed to prepare recharge: {}", e))
    }
}
