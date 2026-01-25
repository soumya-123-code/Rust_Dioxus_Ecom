use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::faqs;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = faqs)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Faq {
    pub id: u64,
    pub question: String,
    pub answer: String,
    pub sort_order: Option<i32>,
    pub status: String,
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<Faq>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let results: Vec<Faq> = faqs::table
        .order(faqs::sort_order.asc())
        .select(Faq::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch FAQs".to_string() }),
            )
        })?;

    Ok(Json(results))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Faq>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let faq: Option<Faq> = faqs::table
        .find(id)
        .select(Faq::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch FAQ".to_string() }),
            )
        })?;

    faq.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "FAQ not found".to_string() }),
        )
    }).map(Json)
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = faqs)]
pub struct CreateFaqRequest {
    pub question: String,
    pub answer: String,
    pub sort_order: Option<i32>,
    pub status: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateFaqRequest>,
) -> Result<(StatusCode, Json<Faq>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::insert_into(faqs::table)
        .values(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create FAQ: {}", e) }),
            )
        })?;

    let created: Faq = faqs::table
        .order(faqs::id.desc())
        .select(Faq::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created FAQ".to_string() }),
            )
        })?;

    Ok((StatusCode::CREATED, Json(created)))
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = faqs)]
pub struct UpdateFaqRequest {
    pub question: Option<String>,
    pub answer: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateFaqRequest>,
) -> Result<Json<Faq>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(faqs::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update FAQ: {}", e) }),
            )
        })?;

    let updated: Faq = faqs::table
        .find(id)
        .select(Faq::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated FAQ".to_string() }),
            )
        })?;

    Ok(Json(updated))
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

    diesel::delete(faqs::table.find(id))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete FAQ".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
