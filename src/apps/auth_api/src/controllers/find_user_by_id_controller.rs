use axum::{extract::{Path, State}, http::StatusCode, response::Json as JsonResponse};
use crate::services::find_user_by_id_service::FindUserByIdService;
use crate::dto::UserResponseDto;
use sea_orm::DatabaseConnection;

pub struct FindUserByIdController;

impl FindUserByIdController {
    pub fn new() -> Self {
        Self
    }
}

pub async fn get_user_by_id(
    Path(user_id): Path<String>,
    State(db): State<DatabaseConnection>,
) -> Result<JsonResponse<UserResponseDto>, StatusCode> {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let service = FindUserByIdService::new(db, redis_url)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = service.find_by_id(&user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(user) => {
            let response: UserResponseDto = user.into();
            Ok(JsonResponse(response))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_user_by_email(
    Path(email): Path<String>,
    State(db): State<DatabaseConnection>,
) -> Result<JsonResponse<UserResponseDto>, StatusCode> {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let service = FindUserByIdService::new(db, redis_url)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = service.find_by_email(&email).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(user) => {
            let response: UserResponseDto = user.into();
            Ok(JsonResponse(response))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
} 