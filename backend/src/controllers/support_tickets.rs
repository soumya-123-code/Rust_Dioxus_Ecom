use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{support_tickets, support_ticket_messages, users, support_ticket_types};
use crate::utils::api_response::{self, ApiResponse};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct SupportTicketResponse {
    pub id: u64,
    pub ticket_number: String,
    pub subject: String,
    pub priority: String,
    pub status: String,
    pub user_name: String,
    pub type_name: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<ApiResponse<Vec<SupportTicketResponse>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Database connection failed".to_string())),
        )
    })?;

    let mut query = support_tickets::table
        .inner_join(users::table.on(support_tickets::user_id.eq(users::id)))
        .inner_join(support_ticket_types::table.on(support_tickets::ticket_type_id.eq(support_ticket_types::id)))
        .select((
            support_tickets::id,
            support_tickets::ticket_number,
            support_tickets::subject,
            support_tickets::priority,
            support_tickets::status,
            support_tickets::created_at,
            users::name,
            support_ticket_types::title,
        ))
        .into_boxed();

    if let Some(status) = params.status {
        query = query.filter(support_tickets::status.eq(status));
    }

    let results = query
        .limit(params.per_page.unwrap_or(10))
        .offset((params.page.unwrap_or(1) - 1) * params.per_page.unwrap_or(10))
        .load::<(u64, String, String, String, String, Option<chrono::NaiveDateTime>, String, String)>(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to fetch tickets: {}", e))),
            )
        })?;

    let response: Vec<SupportTicketResponse> = results
        .into_iter()
        .map(|(id, ticket_number, subject, priority, status, created_at, user_name, type_name)| SupportTicketResponse {
            id,
            ticket_number,
            subject,
            priority,
            status,
            created_at: created_at.map(|d| d.to_string()),
            user_name,
            type_name,
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: u64,
    pub message: String,
    pub is_admin_reply: bool,
    pub created_at: Option<String>,
    pub user_name: String,
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<(SupportTicketResponse, Vec<MessageResponse>)>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Database connection failed".to_string())),
        )
    })?;

    let ticket = support_tickets::table
        .find(id)
        .inner_join(users::table.on(support_tickets::user_id.eq(users::id)))
        .inner_join(support_ticket_types::table.on(support_tickets::ticket_type_id.eq(support_ticket_types::id)))
        .select((
            support_tickets::id,
            support_tickets::ticket_number,
            support_tickets::subject,
            support_tickets::priority,
            support_tickets::status,
            support_tickets::created_at,
            users::name,
            support_ticket_types::title,
        ))
        .first::<(u64, String, String, String, String, Option<chrono::NaiveDateTime>, String, String)>(&mut conn)
        .map_err(|e| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(format!("Ticket not found: {}", e))),
            )
        })?;

    let ticket_response = SupportTicketResponse {
        id: ticket.0,
        ticket_number: ticket.1,
        subject: ticket.2,
        priority: ticket.3,
        status: ticket.4,
        created_at: ticket.5.map(|d| d.to_string()),
        user_name: ticket.6,
        type_name: ticket.7,
    };

    let messages = support_ticket_messages::table
        .filter(support_ticket_messages::ticket_id.eq(id))
        .inner_join(users::table.on(support_ticket_messages::user_id.eq(users::id)))
        .select((
            support_ticket_messages::id,
            support_ticket_messages::message,
            support_ticket_messages::is_admin_reply,
            support_ticket_messages::created_at,
            users::name,
        ))
        .order(support_ticket_messages::created_at.asc())
        .load::<(u64, String, bool, Option<chrono::NaiveDateTime>, String)>(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to fetch messages: {}", e))),
            )
        })?;

    let messages_response: Vec<MessageResponse> = messages
        .into_iter()
        .map(|(id, message, is_admin_reply, created_at, user_name)| MessageResponse {
            id,
            message,
            is_admin_reply,
            created_at: created_at.map(|d| d.to_string()),
            user_name,
        })
        .collect();

    Ok(Json(ApiResponse::success((ticket_response, messages_response))))
}

#[derive(Debug, Deserialize)]
pub struct ReplyRequest {
    pub message: String,
    pub user_id: u64, // Admin user ID
}

#[derive(Insertable)]
#[diesel(table_name = support_ticket_messages)]
pub struct NewMessage {
    pub ticket_id: u64,
    pub user_id: u64,
    pub message: String,
    pub is_admin_reply: bool,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn reply(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<ReplyRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Database connection failed".to_string())),
        )
    })?;

    let new_message = NewMessage {
        ticket_id: id,
        user_id: payload.user_id,
        message: payload.message,
        is_admin_reply: true,
        created_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(support_ticket_messages::table)
        .values(&new_message)
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to send reply: {}", e))),
            )
        })?;

    Ok(Json(ApiResponse::success_message("Reply sent successfully")))
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateStatusRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut conn = state.db_pool.get().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Database connection failed".to_string())),
        )
    })?;

    diesel::update(support_tickets::table.find(id))
        .set(support_tickets::status.eq(&payload.status))
        .execute(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to update ticket status: {}", e))),
            )
        })?;

    Ok(Json(ApiResponse::success_message("Ticket status updated successfully")))
}
