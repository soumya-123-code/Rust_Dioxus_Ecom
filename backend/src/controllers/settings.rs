use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{Setting, NewSetting, SettingResponse};
use crate::schema::settings;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<SettingResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let results: Vec<Setting> = settings::table
        .select(Setting::as_select())
        .load(&mut conn)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch settings".to_string() }),
            )
        })?;

    let response: Vec<SettingResponse> = results.into_iter().map(SettingResponse::from).collect();

    Ok(Json(response))
}

pub async fn show(
    State(state): State<AppState>,
    Path(variable): Path<String>,
) -> Result<Json<SettingResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    let setting: Option<Setting> = settings::table
        .filter(settings::variable.eq(&variable))
        .select(Setting::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message: "Failed to fetch setting".to_string() }),
            )
        })?;

    match setting {
        Some(s) => Ok(Json(SettingResponse::from(s))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { message: "Setting not found".to_string() }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct StoreSettingsRequest {
    pub settings: Vec<SettingInput>,
}

#[derive(Debug, Deserialize)]
pub struct SettingInput {
    pub variable: String,
    pub value: Option<String>,
}

pub async fn store(
    State(state): State<AppState>,
    Json(payload): Json<StoreSettingsRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { message: "Database connection failed".to_string() }),
        )
    })?;

    for input in payload.settings {
        let existing: Option<Setting> = settings::table
            .filter(settings::variable.eq(&input.variable))
            .select(Setting::as_select())
            .first(&mut conn)
            .optional()
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse { message: "Database query failed".to_string() }),
                )
            })?;

        if let Some(s) = existing {
            diesel::update(settings::table.find(s.id))
                .set(settings::value.eq(&input.value))
                .execute(&mut conn)
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse { message: "Failed to update setting".to_string() }),
                    )
                })?;
        } else {
            let new_setting = NewSetting {
                variable: input.variable,
                value: input.value,
            };

            diesel::insert_into(settings::table)
                .values(&new_setting)
                .execute(&mut conn)
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse { message: "Failed to create setting".to_string() }),
                    )
                })?;
        }
    }

    Ok(Json(SuccessResponse {
        success: true,
        message: "Settings saved successfully".to_string(),
    }))
}
