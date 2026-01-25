use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get(&self, endpoint: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let response = self.client.get(&url).send().await?;
        response.json().await
    }

    pub async fn post(&self, endpoint: &str, body: Value) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let response = self.client
            .post(&url)
            .json(&body)
            .send()
            .await?;
        response.json().await
    }

    pub async fn post_with_auth(
        &self,
        endpoint: &str,
        body: Value,
        token: &str,
    ) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await?;
        response.json().await
    }

    pub async fn get_with_auth(
        &self,
        endpoint: &str,
        token: &str,
    ) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        response.json().await
    }

    pub async fn put_with_auth(
        &self,
        endpoint: &str,
        body: Value,
        token: &str,
    ) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let response = self.client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await?;
        response.json().await
    }

    pub async fn delete_with_auth(
        &self,
        endpoint: &str,
        token: &str,
    ) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let response = self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        response.json().await
    }
}
