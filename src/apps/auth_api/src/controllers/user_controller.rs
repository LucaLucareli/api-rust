use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::Json as JsonResponse,
};
use crate::services::user_service::UserService;
use crate::dto::UserResponseDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequestDto {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserResponseDto {
    pub user: UserResponseDto,
    pub message: String,
}

pub struct UserController {
    user_service: UserService,
}

impl UserController {
    pub fn new() -> Self {
        let db = sea_orm::DatabaseConnection::default(); // TODO: Conectar com banco real
        let redis_url = "redis://localhost:6379".to_string();
        
        Self {
            user_service: UserService::new(db, redis_url).expect("Failed to create UserService"),
        }
    }

    pub async fn get_user_by_id(
        Path(user_id): Path<String>,
    ) -> Result<JsonResponse<UserResponseDto>, StatusCode> {
        let controller = Self::new();
        
        match controller.user_service.get_user_by_id(&user_id).await {
            Ok(Some(user)) => {
                let response = UserResponseDto {
                    id: user.id,
                    email: user.email,
                    name: user.name,
                    role: crate::dto::UserRoleDto::from(user.role.as_str()),
                };
                Ok(JsonResponse(response))
            }
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn update_user(
        Path(user_id): Path<String>,
        Json(payload): Json<UpdateUserRequestDto>,
    ) -> Result<JsonResponse<UpdateUserResponseDto>, StatusCode> {
        let controller = Self::new();
        
        match controller.user_service.update_user(&user_id, payload.name, payload.email).await {
            Ok(Some(user)) => {
                let response = UpdateUserResponseDto {
                    user: UserResponseDto {
                        id: user.id,
                        email: user.email,
                        name: user.name,
                        role: crate::dto::UserRoleDto::from(user.role.as_str()),
                    },
                    message: "Usuário atualizado com sucesso".to_string(),
                };
                Ok(JsonResponse(response))
            }
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn delete_user(
        Path(user_id): Path<String>,
    ) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
        let controller = Self::new();
        
        match controller.user_service.delete_user(&user_id).await {
            Ok(true) => {
                let response = serde_json::json!({
                    "message": "Usuário deletado com sucesso"
                });
                Ok(JsonResponse(response))
            }
            Ok(false) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
} 