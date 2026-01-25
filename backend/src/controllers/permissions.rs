use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{permissions, roles, role_has_permissions};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = permissions)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Permission {
    pub id: u64,
    pub name: String,
    pub guard_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub guard_name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = role_has_permissions)]
pub struct RoleHasPermission {
    pub permission_id: u64,
    pub role_id: u64,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Permission>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let mut query = permissions::table.into_boxed();

    if let Some(guard_name) = params.guard_name {
        query = query.filter(permissions::guard_name.eq(guard_name));
    }

    let results: Vec<Permission> = query
        .select(Permission::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch permissions".to_string() }),
            )
        })?;

    Ok(Json(results))
}

pub async fn list_role_permissions(
    State(state): State<AppState>,
    Path(role_id): Path<u64>,
) -> Result<Json<Vec<String>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let permission_names: Vec<String> = role_has_permissions::table
        .inner_join(permissions::table.on(role_has_permissions::permission_id.eq(permissions::id)))
        .filter(role_has_permissions::role_id.eq(role_id))
        .select(permissions::name)
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch role permissions".to_string() }),
            )
        })?;

    Ok(Json(permission_names))
}

#[derive(Debug, Deserialize)]
pub struct StorePermissionRequest {
    pub role: String,
    pub permissions: Vec<String>,
}

pub async fn store(
    State(state): State<AppState>,
    Json(payload): Json<StorePermissionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // 1. Find Role ID
        let role_id: u64 = roles::table
            .filter(roles::name.eq(&payload.role))
            .select(roles::id)
            .first(conn)
            .map_err(|_| diesel::result::Error::NotFound)?;

        // 2. Find Permission IDs
        let permission_ids: Vec<u64> = permissions::table
            .filter(permissions::name.eq_any(&payload.permissions))
            .select(permissions::id)
            .load(conn)?;

        // 3. Delete existing permissions
        diesel::delete(role_has_permissions::table.filter(role_has_permissions::role_id.eq(role_id)))
            .execute(conn)?;

        // 4. Insert new permissions
        if !permission_ids.is_empty() {
            let records: Vec<RoleHasPermission> = permission_ids
                .into_iter()
                .map(|pid| RoleHasPermission {
                    permission_id: pid,
                    role_id: role_id,
                })
                .collect();

            diesel::insert_into(role_has_permissions::table)
                .values(&records)
                .execute(conn)?;
        }

        Ok(())
    }).map_err(|e| {
        let (status, msg) = match e {
            diesel::result::Error::NotFound => (StatusCode::NOT_FOUND, "Role not found".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update permissions: {}", e)),
        };
        (
            status,
            Json(ErrorResponse { message: msg }),
        )
    })?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Permissions updated successfully"
    })))
}
