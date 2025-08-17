use axum::{extract::{Query, State}, http::StatusCode, response::Json as JsonResponse};
use crate::services::find_users_service::FindUsersService;
use crate::dto::UserResponseDto;
use serde::{Deserialize, Serialize};
use sea_orm::DatabaseConnection;

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct RoleQuery {
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct ListUsersResponseDto {
    pub users: Vec<UserResponseDto>,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct CountUsersResponseDto {
    pub count: u64,
}

pub struct FindUsersController;

impl FindUsersController {
    pub fn new() -> Self {
        Self
    }
}

pub async fn list_users(
    Query(query): Query<ListUsersQuery>,
    State(db): State<DatabaseConnection>,
) -> Result<JsonResponse<ListUsersResponseDto>, StatusCode> {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let service = FindUsersService::new(db, redis_url)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users = service.find_all(query.limit, query.offset).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = service.count().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ListUsersResponseDto {
        users: users.into_iter().map(|u| u.into()).collect(),
        total,
    };

    Ok(JsonResponse(response))
}

pub async fn get_users_by_role(
    Query(query): Query<RoleQuery>,
    State(db): State<DatabaseConnection>,
) -> Result<JsonResponse<Vec<UserResponseDto>>, StatusCode> {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let service = FindUsersService::new(db, redis_url)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users = service.find_by_role(&query.role).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<UserResponseDto> = users.into_iter().map(|u| u.into()).collect();
    Ok(JsonResponse(response))
}

pub async fn count_users(
    State(db): State<DatabaseConnection>,
) -> Result<JsonResponse<CountUsersResponseDto>, StatusCode> {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let service = FindUsersService::new(db, redis_url)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let count = service.count().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = CountUsersResponseDto { count };
    Ok(JsonResponse(response))
} 