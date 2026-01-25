use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::roles;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = roles)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Role {
    pub id: u64,
    pub team_id: Option<u64>,
    pub name: String,
    pub guard_name: String,
    pub label: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub guard_name: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Role>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = roles::table.into_boxed();

    if let Some(guard_name) = params.guard_name {
        query = query.filter(roles::guard_name.eq(guard_name));
    }

    let results: Vec<Role> = query
        .select(Role::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch roles".to_string() }),
            )
        })?;

    Ok(Json(results))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Role>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let role: Option<Role> = roles::table
        .find(id)
        .select(Role::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch role".to_string() }),
            )
        })?;

    role.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Role not found".to_string() }),
        )
    }).map(Json)
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = roles)]
pub struct CreateRoleRequest {
    pub name: String,
    pub guard_name: String,
    pub label: Option<String>,
    pub description: Option<String>,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<Role>), (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    if ["Super Admin", "customer", "seller"].contains(&payload.name.as_str()) {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse { message: "Cannot create role with this name".to_string() }),
        ));
    }

    diesel::insert_into(roles::table)
        .values(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to create role: {}", e) }),
            )
        })?;

    let created: Role = roles::table
        .order(roles::id.desc())
        .select(Role::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve created role".to_string() }),
            )
        })?;

    Ok((StatusCode::CREATED, Json(created)))
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = roles)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub label: Option<String>,
    pub description: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<Role>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    diesel::update(roles::table.find(id))
        .set(&payload)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { message: format!("Failed to update role: {}", e) }),
            )
        })?;

    let updated: Role = roles::table
        .find(id)
        .select(Role::as_select())
        .first(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to retrieve updated role".to_string() }),
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

    diesel::delete(roles::table.find(id))
        .execute(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to delete role".to_string() }),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
