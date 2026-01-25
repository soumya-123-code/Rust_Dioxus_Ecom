use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub image: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "sub_categories")]
    pub sub_categories: Option<Vec<SubCategory>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubCategory {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub image: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryListResponse {
    pub success: Option<bool>,
    pub data: Option<Vec<Category>>,
    pub message: Option<String>,
}
