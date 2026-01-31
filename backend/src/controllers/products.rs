use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{NewProduct, Product, ProductResponse, UpdateProduct};
use crate::schema::products;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<ProductResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub verification_status: Option<String>,
    pub category_id: Option<u64>,
    pub seller_id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<ProductResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = products::table
        .filter(products::deleted_at.is_null())
        .into_boxed();

    if let Some(status) = params.status {
        query = query.filter(products::status.eq(status));
    }

    if let Some(verification_status) = params.verification_status {
        query = query.filter(products::verification_status.eq(verification_status));
    }

    if let Some(search) = params.search {
        query = query.filter(products::name.like(format!("%{}%", search)));
    }

    if let Some(category_id) = params.category_id {
        query = query.filter(products::category_id.eq(category_id));
    }

    if let Some(seller_id) = params.seller_id {
        query = query.filter(products::seller_id.eq(seller_id));
    }

    let results: Vec<Product> = query
        .order(products::created_at.desc())
        .select(Product::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch products".to_string() }),
            )
        })?;

    let response: Vec<ProductResponse> = results.into_iter().map(ProductResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<ProductResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let mut query = products::table
        .filter(products::deleted_at.is_null())
        .into_boxed();

    let mut count_query = products::table
        .filter(products::deleted_at.is_null())
        .into_boxed();

    if let Some(status) = &params.status {
        query = query.filter(products::status.eq(status));
        count_query = count_query.filter(products::status.eq(status));
    }

    if let Some(verification_status) = &params.verification_status {
        query = query.filter(products::verification_status.eq(verification_status));
        count_query = count_query.filter(products::verification_status.eq(verification_status));
    }

    if let Some(search) = &params.search {
        query = query.filter(products::name.like(format!("%{}%", search)));
        count_query = count_query.filter(products::name.like(format!("%{}%", search)));
    }

    if let Some(category_id) = params.category_id {
        query = query.filter(products::category_id.eq(category_id));
        count_query = count_query.filter(products::category_id.eq(category_id));
    }

    if let Some(seller_id) = params.seller_id {
        query = query.filter(products::seller_id.eq(seller_id));
        count_query = count_query.filter(products::seller_id.eq(seller_id));
    }

    let total: i64 = count_query
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<Product> = query
        .order(products::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .select(Product::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch products".to_string() }),
            )
        })?;

    let data: Vec<ProductResponse> = results.into_iter().map(ProductResponse::from).collect();

    Ok(Json(PaginatedResponse {
        data,
        total,
        page,
        per_page,
    }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let product: Option<Product> = products::table
        .find(id)
        .filter(products::deleted_at.is_null())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch product".to_string() }),
            )
        })?;

    match product {
        Some(p) => Ok(Json(SuccessResponse {
            success: true,
            message: "Product retrieved successfully".to_string(),
            data: Some(ProductResponse::from(p)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Product not found".to_string() }),
        )),
    }
}

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<ProductResponse>>, (StatusCode, Json<ErrorResponse>)> {
    list(State(state), Query(params)).await
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub seller_id: u64,
    pub category_id: u64,
    pub brand_id: Option<u64>,
    pub tax_class_id: Option<u64>,
    pub name: String,
    pub short_description: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub hsn_code: Option<String>,
    pub weight: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub length: Option<Decimal>,
    pub width: Option<Decimal>,
    pub height: Option<Decimal>,
    pub dimension_unit: Option<String>,
    pub price: Decimal,
    pub sale_price: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub is_taxable: Option<bool>,
    pub tax_included: Option<bool>,
    pub min_order_quantity: Option<i32>,
    pub max_order_quantity: Option<i32>,
    pub quantity_step: Option<i32>,
    pub stock_quantity: Option<i32>,
    pub low_stock_threshold: Option<i32>,
    pub stock_status: Option<String>,
    pub manage_stock: Option<bool>,
    pub sold_individually: Option<bool>,
    pub is_returnable: Option<bool>,
    pub return_days: Option<i32>,
    pub is_cancelable: Option<bool>,
    pub cancel_till_status: Option<String>,
    pub is_cod_available: Option<bool>,
    pub preparation_time: Option<i32>,
    pub video_url: Option<String>,
    pub external_url: Option<String>,
    pub download_url: Option<String>,
    pub download_limit: Option<i32>,
    pub download_expiry_days: Option<i32>,
    pub warranty_info: Option<String>,
    pub country_of_origin: Option<String>,
    pub verification_status: Option<String>,
    pub verification_note: Option<String>,
    pub is_featured: Option<bool>,
    pub is_bestseller: Option<bool>,
    pub is_new_arrival: Option<bool>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub seo_keywords: Option<String>,
    pub tags: Option<String>,
    pub metadata: Option<String>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let slug = generate_slug(&payload.name);
    let now = Utc::now().naive_utc();
    let uuid = Uuid::new_v4().to_string();

    let new_product = NewProduct {
        uuid,
        seller_id: payload.seller_id,
        category_id: payload.category_id,
        brand_id: payload.brand_id,
        tax_class_id: payload.tax_class_id,
        name: payload.name,
        slug,
        short_description: payload.short_description,
        description: payload.description,
        type_: payload.type_,
        sku: payload.sku,
        barcode: payload.barcode,
        hsn_code: payload.hsn_code,
        weight: payload.weight,
        weight_unit: payload.weight_unit,
        length: payload.length,
        width: payload.width,
        height: payload.height,
        dimension_unit: payload.dimension_unit,
        base_price: payload.price,
        sale_price: payload.sale_price,
        cost_price: payload.cost_price,
        is_taxable: payload.is_taxable.map(|v| if v { 1 } else { 0 }),
        tax_included: payload.tax_included.map(|v| if v { 1 } else { 0 }),
        min_order_quantity: payload.min_order_quantity,
        max_order_quantity: payload.max_order_quantity,
        quantity_step: payload.quantity_step,
        stock_quantity: payload.stock_quantity,
        low_stock_threshold: payload.low_stock_threshold,
        stock_status: payload.stock_status,
        manage_stock: payload.manage_stock.map(|v| if v { 1 } else { 0 }),
        sold_individually: payload.sold_individually.map(|v| if v { 1 } else { 0 }),
        is_returnable: payload.is_returnable.map(|v| if v { 1 } else { 0 }),
        return_days: payload.return_days,
        is_cancelable: payload.is_cancelable.map(|v| if v { 1 } else { 0 }),
        cancel_till_status: payload.cancel_till_status,
        is_cod_available: payload.is_cod_available.map(|v| if v { 1 } else { 0 }),
        preparation_time: payload.preparation_time,
        video_url: payload.video_url,
        external_url: payload.external_url,
        download_url: payload.download_url,
        download_limit: payload.download_limit,
        download_expiry_days: payload.download_expiry_days,
        warranty_info: payload.warranty_info,
        country_of_origin: payload.country_of_origin,
        verification_status: payload.verification_status,
        verification_note: payload.verification_note,
        is_featured: payload.is_featured.map(|v| if v { 1 } else { 0 }),
        is_bestseller: payload.is_bestseller.map(|v| if v { 1 } else { 0 }),
        is_new_arrival: payload.is_new_arrival.map(|v| if v { 1 } else { 0 }),
        sort_order: payload.sort_order,
        status: payload.status,
        seo_title: payload.seo_title,
        seo_description: payload.seo_description,
        seo_keywords: payload.seo_keywords,
        tags: payload.tags,
        metadata: payload.metadata,
        published_at: None,
        created_at: now,
        updated_at: now,
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: format!("Failed to create product: {}", e) }),
            )
        })?;
    
    let created_product: Product = products::table
        .order(products::id.desc())
        .first(&mut conn)
        .map_err(|_| {
             (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created product".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Product created successfully".to_string(),
        data: Some(ProductResponse::from(created_product)),
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub category_id: Option<u64>,
    pub brand_id: Option<u64>,
    pub tax_class_id: Option<u64>,
    pub name: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub hsn_code: Option<String>,
    pub weight: Option<Decimal>,
    pub weight_unit: Option<String>,
    pub length: Option<Decimal>,
    pub width: Option<Decimal>,
    pub height: Option<Decimal>,
    pub dimension_unit: Option<String>,
    pub price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub is_taxable: Option<bool>,
    pub tax_included: Option<bool>,
    pub min_order_quantity: Option<i32>,
    pub max_order_quantity: Option<i32>,
    pub quantity_step: Option<i32>,
    pub stock_quantity: Option<i32>,
    pub low_stock_threshold: Option<i32>,
    pub stock_status: Option<String>,
    pub manage_stock: Option<bool>,
    pub sold_individually: Option<bool>,
    pub is_returnable: Option<bool>,
    pub return_days: Option<i32>,
    pub is_cancelable: Option<bool>,
    pub cancel_till_status: Option<String>,
    pub is_cod_available: Option<bool>,
    pub preparation_time: Option<i32>,
    pub video_url: Option<String>,
    pub external_url: Option<String>,
    pub download_url: Option<String>,
    pub download_limit: Option<i32>,
    pub download_expiry_days: Option<i32>,
    pub warranty_info: Option<String>,
    pub country_of_origin: Option<String>,
    pub verification_status: Option<String>,
    pub verification_note: Option<String>,
    pub is_featured: Option<bool>,
    pub is_bestseller: Option<bool>,
    pub is_new_arrival: Option<bool>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub seo_keywords: Option<String>,
    pub tags: Option<String>,
    pub metadata: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let slug = payload.name.as_ref().map(|name| generate_slug(name));
    let now = Utc::now().naive_utc();

    let update_product = UpdateProduct {
        category_id: payload.category_id,
        brand_id: payload.brand_id,
        tax_class_id: payload.tax_class_id,
        name: payload.name,
        slug,
        short_description: payload.short_description,
        description: payload.description,
        type_: payload.type_,
        sku: payload.sku,
        barcode: payload.barcode,
        hsn_code: payload.hsn_code,
        weight: payload.weight,
        weight_unit: payload.weight_unit,
        length: payload.length,
        width: payload.width,
        height: payload.height,
        dimension_unit: payload.dimension_unit,
        base_price: payload.price,
        sale_price: payload.sale_price,
        cost_price: payload.cost_price,
        is_taxable: payload.is_taxable.map(|v| if v { 1 } else { 0 }),
        tax_included: payload.tax_included.map(|v| if v { 1 } else { 0 }),
        min_order_quantity: payload.min_order_quantity,
        max_order_quantity: payload.max_order_quantity,
        quantity_step: payload.quantity_step,
        stock_quantity: payload.stock_quantity,
        low_stock_threshold: payload.low_stock_threshold,
        stock_status: payload.stock_status,
        manage_stock: payload.manage_stock.map(|v| if v { 1 } else { 0 }),
        sold_individually: payload.sold_individually.map(|v| if v { 1 } else { 0 }),
        is_returnable: payload.is_returnable.map(|v| if v { 1 } else { 0 }),
        return_days: payload.return_days,
        is_cancelable: payload.is_cancelable.map(|v| if v { 1 } else { 0 }),
        cancel_till_status: payload.cancel_till_status,
        is_cod_available: payload.is_cod_available.map(|v| if v { 1 } else { 0 }),
        preparation_time: payload.preparation_time,
        video_url: payload.video_url,
        external_url: payload.external_url,
        download_url: payload.download_url,
        download_limit: payload.download_limit,
        download_expiry_days: payload.download_expiry_days,
        warranty_info: payload.warranty_info,
        country_of_origin: payload.country_of_origin,
        verification_status: payload.verification_status,
        verification_note: payload.verification_note,
        is_featured: payload.is_featured.map(|v| if v { 1 } else { 0 }),
        is_bestseller: payload.is_bestseller.map(|v| if v { 1 } else { 0 }),
        is_new_arrival: payload.is_new_arrival.map(|v| if v { 1 } else { 0 }),
        sort_order: payload.sort_order,
        status: payload.status,
        seo_title: payload.seo_title,
        seo_description: payload.seo_description,
        seo_keywords: payload.seo_keywords,
        tags: payload.tags,
        metadata: payload.metadata,
        published_at: None,
        updated_at: Some(now),
    };

    diesel::update(products::table.find(id))
        .set(&update_product)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: format!("Failed to update product: {}", e) }),
            )
        })?;

    let updated_product: Product = products::table
        .find(id)
        .first(&mut conn)
        .map_err(|_| {
             (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated product".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Product updated successfully".to_string(),
        data: Some(ProductResponse::from(updated_product)),
    }))
}

fn generate_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

#[derive(Debug, Deserialize)]
pub struct UpdateVerificationStatusRequest {
    pub verification_status: String,
    pub rejection_reason: Option<String>,
}

pub async fn update_verification_status(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateVerificationStatusRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(products::table.find(id))
        .set((
            products::verification_status.eq(&payload.verification_status),
            products::verification_note.eq(&payload.rejection_reason),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update verification status: {}", e) }),
            )
        })?;

    let updated: Product = products::table
        .find(id)
        .select(Product::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated product".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Verification status updated successfully".to_string(),
        data: Some(ProductResponse::from(updated)),
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateStatusRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(products::table.find(id))
        .set(products::status.eq(&payload.status))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update status: {}", e) }),
            )
        })?;

    let updated: Product = products::table
        .find(id)
        .select(Product::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated product".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Status updated successfully".to_string(),
        data: Some(ProductResponse::from(updated)),
    }))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    // Soft delete by setting deleted_at
    diesel::update(products::table.find(id))
        .set(products::deleted_at.eq(chrono::Utc::now().naive_utc()))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete product".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
