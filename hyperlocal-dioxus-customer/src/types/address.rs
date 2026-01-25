use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: Option<i32>,
    #[serde(rename = "address_line1")]
    pub address_line1: Option<String>,
    #[serde(rename = "address_line2")]
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub landmark: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(rename = "address_type")]
    pub address_type: Option<String>,
    #[serde(rename = "is_default")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressRequest {
    #[serde(rename = "address_line1")]
    pub address_line1: String,
    #[serde(rename = "address_line2")]
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub landmark: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "address_type")]
    pub address_type: String,
    #[serde(rename = "is_default")]
    pub is_default: Option<bool>,
}
