use axum::{extract::{Json, State}, http::StatusCode, response::Json as JsonResponse};
use crate::services::create_user_service::CreateUserService;
use crate::dto::UserResponseDto;
use serde::{Deserialize, Serialize};
use sea_orm::DatabaseConnection;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequestDto {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponseDto {
    pub user: UserResponseDto,
    pub message: String,
}

pub struct CreateUserController;

impl CreateUserController {
    pub fn new() -> Self {
        Self
    }
}

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateUserRequestDto>,
) -> Result<JsonResponse<CreateUserResponseDto>, StatusCode> {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let service = CreateUserService::new(db, redis_url)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = service.create_user(&payload.email, &payload.password, &payload.name, payload.role).await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let response = CreateUserResponseDto {
        user: user.into(),
        message: "Usuário criado com sucesso".to_string(),
    };

    Ok(JsonResponse(response))
} 