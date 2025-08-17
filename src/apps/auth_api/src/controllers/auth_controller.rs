use axum::{extract::{Json, State}, http::StatusCode, response::Json as JsonResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use crate::dto::{LoginRequestDto, RegisterRequestDto, TokenPair};
use crate::services::auth_service::AuthService;

pub struct AuthController;

impl AuthController {
    pub async fn health() -> JsonResponse<&'static str> {
        JsonResponse("Auth API - OK")
    }
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginRequestDto>,
) -> Result<JsonResponse<TokenPair>, StatusCode> {
    let jwt_secret = std::env::var("JWT_ACCESS_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_service = AuthService::new(db, jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token_pair = auth_service.login(&payload.email, &payload.password).await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(JsonResponse(token_pair))
}

pub async fn register(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<RegisterRequestDto>,
) -> Result<JsonResponse<TokenPair>, StatusCode> {
    let jwt_secret = std::env::var("JWT_ACCESS_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_service = AuthService::new(db, jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token_pair = auth_service.register(&payload.email, &payload.password, &payload.name, None).await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(JsonResponse(token_pair))
}

pub async fn refresh_token(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<RefreshTokenRequestDto>,
) -> Result<JsonResponse<TokenPair>, StatusCode> {
    let jwt_secret = std::env::var("JWT_ACCESS_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_service = AuthService::new(db, jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token_pair = auth_service.refresh_token(&payload.refresh_token).await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(JsonResponse(token_pair))
}

#[derive(Deserialize)]
pub struct RefreshTokenRequestDto {
    pub refresh_token: String,
} 