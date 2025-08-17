use sea_orm::DatabaseConnection;
use crate::models::user::User;
use crate::services::user_service::UserService;
use chrono::Utc;

pub struct CreateUserService {
    user_service: UserService,
}

impl CreateUserService {
    pub fn new(db: DatabaseConnection, redis_url: String) -> Result<Self, redis::RedisError> {
        let user_service = UserService::new(db, redis_url)?;
        Ok(Self { user_service })
    }

    pub async fn create_user(&self, email: &str, password: &str, name: &str, role: Option<String>) -> Result<User, String> {
        // Por enquanto, criar usuário mock
        // TODO: Implementar criação real no banco
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            email: email.to_string(),
            name: name.to_string(),
            role: role.unwrap_or_else(|| "viewer".to_string()),
            password_hash: "hash".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(user)
    }
} 