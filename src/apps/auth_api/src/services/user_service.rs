use sea_orm::DatabaseConnection;
use crate::models::user::User;
use crate::dto::{UserRoleDto, CreateUserRequestDto};
use chrono::Utc;
use uuid::Uuid;

// Importar o repositório de usuários
use api_rust::libs::shared::database::repositories::users::{UsersRepository, CreateUserRequest, UpdateUserRequest};

pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection, _redis_url: String) -> Result<Self, redis::RedisError> {
        Ok(Self { db })
    }

    pub async fn list_users(&self, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<User>, String> {
        let users_repo = UsersRepository::new(self.db.clone());
        
        match users_repo.find_all(limit, offset).await {
            Ok(users) => {
                // Converter do repositório para o modelo da API
                let api_users: Vec<User> = users.into_iter().map(|repo_user| User {
                    id: repo_user.id,
                    email: repo_user.email,
                    name: repo_user.name,
                    role: repo_user.role,
                    password_hash: repo_user.password_hash,
                    created_at: repo_user.created_at,
                    updated_at: repo_user.updated_at,
                }).collect();
                
                Ok(api_users)
            }
            Err(e) => Err(format!("Erro ao buscar usuários: {}", e)),
        }
    }

    pub async fn count_users(&self) -> Result<u64, String> {
        let users_repo = UsersRepository::new(self.db.clone());
        
        match users_repo.count().await {
            Ok(count) => Ok(count),
            Err(e) => Err(format!("Erro ao contar usuários: {}", e)),
        }
    }

    pub async fn get_users_by_role(&self, role: &str) -> Result<Vec<User>, String> {
        let users_repo = UsersRepository::new(self.db.clone());
        
        match users_repo.find_by_role(role).await {
            Ok(users) => {
                // Converter do repositório para o modelo da API
                let api_users: Vec<User> = users.into_iter().map(|repo_user| User {
                    id: repo_user.id,
                    email: repo_user.email,
                    name: repo_user.name,
                    role: repo_user.role,
                    password_hash: repo_user.password_hash,
                    created_at: repo_user.created_at,
                    updated_at: repo_user.updated_at,
                }).collect();
                
                Ok(api_users)
            }
            Err(e) => Err(format!("Erro ao buscar usuários por role: {}", e)),
        }
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, String> {
        let users_repo = UsersRepository::new(self.db.clone());
        
        match users_repo.find_by_id(user_id).await {
            Ok(Some(repo_user)) => {
                // Converter do repositório para o modelo da API
                let api_user = User {
                    id: repo_user.id,
                    email: repo_user.email,
                    name: repo_user.name,
                    role: repo_user.role,
                    password_hash: repo_user.password_hash,
                    created_at: repo_user.created_at,
                    updated_at: repo_user.updated_at,
                };
                
                Ok(Some(api_user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Erro ao buscar usuário: {}", e)),
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let users_repo = UsersRepository::new(self.db.clone());
        
        match users_repo.find_by_email(email).await {
            Ok(Some(repo_user)) => {
                // Converter do repositório para o modelo da API
                let api_user = User {
                    id: repo_user.id,
                    email: repo_user.email,
                    name: repo_user.name,
                    role: repo_user.role,
                    password_hash: repo_user.password_hash,
                    created_at: repo_user.created_at,
                    updated_at: repo_user.updated_at,
                };
                
                Ok(Some(api_user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Erro ao buscar usuário por email: {}", e)),
        }
    }

    pub async fn create_user(&self, request: CreateUserRequestDto) -> Result<User, String> {
        let users_repo = UsersRepository::new(self.db.clone());
        
        // Converter DTO da API para DTO do repositório
        let repo_request = CreateUserRequest {
            email: request.email,
            password: request.password,
            name: request.name,
            role: request.role,
        };
        
        match users_repo.create(repo_request).await {
            Ok(repo_user) => {
                // Converter do repositório para o modelo da API
                let api_user = User {
                    id: repo_user.id,
                    email: repo_user.email,
                    name: repo_user.name,
                    role: repo_user.role,
                    password_hash: repo_user.password_hash,
                    created_at: repo_user.created_at,
                    updated_at: repo_user.updated_at,
                };
                
                Ok(api_user)
            }
            Err(e) => Err(format!("Erro ao criar usuário: {}", e)),
        }
    }

    pub async fn update_user(&self, user_id: &str, name: Option<String>, email: Option<String>) -> Result<Option<User>, String> {
        let users_repo = UsersRepository::new(self.db.clone());
        
        // Converter para DTO do repositório
        let repo_request = UpdateUserRequest {
            name,
            email,
            role: None, // Não permitir alterar role via update
        };
        
        match users_repo.update(user_id, repo_request).await {
            Ok(Some(repo_user)) => {
                // Converter do repositório para o modelo da API
                let api_user = User {
                    id: repo_user.id,
                    email: repo_user.email,
                    name: repo_user.name,
                    role: repo_user.role,
                    password_hash: repo_user.password_hash,
                    created_at: repo_user.created_at,
                    updated_at: repo_user.updated_at,
                };
                
                Ok(Some(api_user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Erro ao atualizar usuário: {}", e)),
        }
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<bool, String> {
        let users_repo = UsersRepository::new(self.db.clone());
        
        match users_repo.delete(user_id).await {
            Ok(success) => Ok(success),
            Err(e) => Err(format!("Erro ao deletar usuário: {}", e)),
        }
    }
} 