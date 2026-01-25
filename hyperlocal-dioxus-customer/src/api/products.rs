use crate::types::product::{Banner, Product, ProductListResponse};
use crate::api::client::ApiClient;
use serde_json::Value;

#[derive(Clone)]
pub struct ProductsApi {
    api: ApiClient,
}

impl ProductsApi {
    pub fn new(base_url: String) -> Self {
        Self {
            api: ApiClient::new(base_url),
        }
    }

    pub async fn get_categories(&self) -> Result<Value, String> {
        self.api
            .get("categories")
            .await
            .map_err(|e| format!("Failed to fetch categories: {}", e))
    }

    pub async fn get_banners(&self, category_slug: Option<&str>) -> Result<Vec<Banner>, String> {
        let endpoint = if let Some(slug) = category_slug {
            format!("banners?category_slug={}", slug)
        } else {
            "banners".to_string()
        };

        match self.api.get(&endpoint).await {
            Ok(response) => {
                // Parse banners from response
                Ok(vec![])
            }
            Err(e) => Err(format!("Failed to fetch banners: {}", e)),
        }
    }

    pub async fn get_products(
        &self,
        category_slug: Option<&str>,
        token: Option<&str>,
    ) -> Result<ProductListResponse, String> {
        let endpoint = if let Some(slug) = category_slug {
            format!("delivery-zone/products?category_slug={}", slug)
        } else {
            "delivery-zone/products".to_string()
        };

        let response = if let Some(t) = token {
            self.api.get_with_auth(&endpoint, t).await
        } else {
            self.api.get(&endpoint).await
        };

        match response {
            Ok(data) => {
                serde_json::from_value(data)
                    .map_err(|e| format!("Failed to parse products: {}", e))
            }
            Err(e) => Err(format!("Failed to fetch products: {}", e)),
        }
    }

    pub async fn get_product_detail(
        &self,
        product_slug: &str,
        token: Option<&str>,
    ) -> Result<Product, String> {
        let endpoint = format!("products/{}", product_slug);

        let response = if let Some(t) = token {
            self.api.get_with_auth(&endpoint, t).await
        } else {
            self.api.get(&endpoint).await
        };

        match response {
            Ok(data) => {
                serde_json::from_value(data)
                    .map_err(|e| format!("Failed to parse product: {}", e))
            }
            Err(e) => Err(format!("Failed to fetch product: {}", e)),
        }
    }
}
