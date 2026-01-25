use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{Category, NewCategory, UpdateCategory, CategoryResponse};
use crate::schema::categories;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<CategoryResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub status: Option<String>,
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
) -> Result<Json<Vec<CategoryResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = categories::table
        .filter(categories::deleted_at.is_null())
        .into_boxed();

    if let Some(status) = params.status {
        query = query.filter(categories::status.eq(status));
    }

    if let Some(search) = params.search {
        query = query.filter(categories::title.like(format!("%{}%", search)));
    }

    let results: Vec<Category> = query
        .order(categories::created_at.desc())
        .select(Category::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch categories".to_string() }),
            )
        })?;

    let response: Vec<CategoryResponse> = results.into_iter().map(CategoryResponse::from).collect();

    Ok(Json(response))
}

pub async fn datatable(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PaginatedResponse<CategoryResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let mut query = categories::table
        .filter(categories::deleted_at.is_null())
        .into_boxed();

    if let Some(status) = params.status {
        query = query.filter(categories::status.eq(status));
    }

    if let Some(search) = &params.search {
        query = query.filter(categories::title.like(format!("%{}%", search)));
    }

    let total: i64 = categories::table
        .filter(categories::deleted_at.is_null())
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    let results: Vec<Category> = query
        .order(categories::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .select(Category::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch categories".to_string() }),
            )
        })?;

    let data: Vec<CategoryResponse> = results.into_iter().map(CategoryResponse::from).collect();

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

    let category: Option<Category> = categories::table
        .find(id)
        .filter(categories::deleted_at.is_null())
        .select(Category::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch category".to_string() }),
            )
        })?;

    match category {
        Some(cat) => Ok(Json(SuccessResponse {
            success: true,
            message: "Category retrieved successfully".to_string(),
            data: Some(CategoryResponse::from(cat)),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Category not found".to_string() }),
        )),
    }
}

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<CategoryResponse>>, (StatusCode, Json<ErrorResponse>)> {
    list(State(state), Query(params)).await
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub parent_id: Option<u64>,
    pub title: String,
    pub image: String,
    pub banner: Option<String>,
    pub description: String,
    pub status: Option<String>,
    pub requires_approval: Option<bool>,
    pub commission: Option<rust_decimal::Decimal>,
    pub background_type: Option<String>,
    pub background_color: Option<String>,
    pub font_color: Option<String>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<SuccessResponse>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let slug = generate_slug(&payload.title);
    let uuid = Uuid::new_v4().to_string();

    let new_category = NewCategory {
        uuid,
        parent_id: payload.parent_id,
        title: payload.title,
        slug,
        image: payload.image,
        banner: payload.banner,
        description: payload.description,
        status: payload.status.unwrap_or_else(|| "inactive".to_string()),
        requires_approval: payload.requires_approval.unwrap_or(false),
        commission: payload.commission,
        background_type: payload.background_type,
        background_color: payload.background_color,
        font_color: payload.font_color,
        metadata: serde_json::json!({}),
    };

    diesel::insert_into(categories::table)
        .values(&new_category)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create category: {}", e) }),
            )
        })?;

    // Get the created category
    let created: Category = categories::table
        .order(categories::id.desc())
        .select(Category::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created category".to_string() }),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse {
            success: true,
            message: "Category created successfully".to_string(),
            data: Some(CategoryResponse::from(created)),
        }),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateCategory>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    // Check if category exists
    let existing: Option<Category> = categories::table
        .find(id)
        .filter(categories::deleted_at.is_null())
        .select(Category::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Database query failed".to_string() }),
            )
        })?;

    if existing.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Category not found".to_string() }),
        ));
    }

    diesel::update(categories::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update category: {}", e) }),
            )
        })?;

    let updated: Category = categories::table
        .find(id)
        .select(Category::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated category".to_string() }),
            )
        })?;

    Ok(Json(SuccessResponse {
        success: true,
        message: "Category updated successfully".to_string(),
        data: Some(CategoryResponse::from(updated)),
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
    diesel::update(categories::table.find(id))
        .set(categories::deleted_at.eq(chrono::Utc::now().naive_utc()))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete category".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
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
