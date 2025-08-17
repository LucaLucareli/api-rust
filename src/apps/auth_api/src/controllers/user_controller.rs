use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Json as JsonResponse,
};
use sea_orm::DatabaseConnection;
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

pub struct UserController;

impl UserController {
    pub async fn list_users(
        State(db): State<DatabaseConnection>,
    ) -> Result<JsonResponse<Vec<UserResponseDto>>, StatusCode> {
        let user_service = UserService::new(db, "redis://localhost:6379".to_string())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        match user_service.list_users(None, None).await {
            Ok(users) => {
                let response: Vec<UserResponseDto> = users
                    .into_iter()
                    .map(|user| UserResponseDto {
                        id: user.id,
                        email: user.email,
                        name: user.name,
                        role: crate::dto::UserRoleDto::from(user.role.as_str()),
                    })
                    .collect();
                Ok(JsonResponse(response))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn count_users(
        State(db): State<DatabaseConnection>,
    ) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
        let user_service = UserService::new(db, "redis://localhost:6379".to_string())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        match user_service.count_users().await {
            Ok(count) => {
                let response = serde_json::json!({
                    "total_users": count
                });
                Ok(JsonResponse(response))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_users_by_role(
        State(db): State<DatabaseConnection>,
        Path(role): Path<String>,
    ) -> Result<JsonResponse<Vec<UserResponseDto>>, StatusCode> {
        let user_service = UserService::new(db, "redis://localhost:6379".to_string())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        match user_service.get_users_by_role(&role).await {
            Ok(users) => {
                let response: Vec<UserResponseDto> = users
                    .into_iter()
                    .map(|user| UserResponseDto {
                        id: user.id,
                        email: user.email,
                        name: user.name,
                        role: crate::dto::UserRoleDto::from(user.role.as_str()),
                    })
                    .collect();
                Ok(JsonResponse(response))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_user_by_id(
        State(db): State<DatabaseConnection>,
        Path(user_id): Path<String>,
    ) -> Result<JsonResponse<UserResponseDto>, StatusCode> {
        let user_service = UserService::new(db, "redis://localhost:6379".to_string())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        match user_service.get_user_by_id(&user_id).await {
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

    pub async fn get_user_by_email(
        State(db): State<DatabaseConnection>,
        Path(email): Path<String>,
    ) -> Result<JsonResponse<UserResponseDto>, StatusCode> {
        let user_service = UserService::new(db, "redis://localhost:6379".to_string())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        match user_service.get_user_by_email(&email).await {
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

    pub async fn create_user(
        State(db): State<DatabaseConnection>,
        Json(payload): Json<crate::dto::CreateUserRequestDto>,
    ) -> Result<JsonResponse<UserResponseDto>, StatusCode> {
        let user_service = UserService::new(db, "redis://localhost:6379".to_string())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        match user_service.create_user(payload).await {
            Ok(user) => {
                let response = UserResponseDto {
                    id: user.id,
                    email: user.email,
                    name: user.name,
                    role: crate::dto::UserRoleDto::from(user.role.as_str()),
                };
                Ok(JsonResponse(response))
            }
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    pub async fn update_user(
        State(db): State<DatabaseConnection>,
        Path(user_id): Path<String>,
        Json(payload): Json<UpdateUserRequestDto>,
    ) -> Result<JsonResponse<UpdateUserResponseDto>, StatusCode> {
        let user_service = UserService::new(db, "redis://localhost:6379".to_string())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        match user_service.update_user(&user_id, payload.name, payload.email).await {
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
        State(db): State<DatabaseConnection>,
        Path(user_id): Path<String>,
    ) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
        let user_service = UserService::new(db, "redis://localhost:6379".to_string())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        match user_service.delete_user(&user_id).await {
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