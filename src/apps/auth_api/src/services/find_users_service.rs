use sea_orm::DatabaseConnection;
use crate::models::user::User;
use crate::services::user_service::UserService;

pub struct FindUsersService {
    user_service: UserService,
}

impl FindUsersService {
    pub fn new(db: DatabaseConnection, redis_url: String) -> Result<Self, redis::RedisError> {
        let user_service = UserService::new(db, redis_url)?;
        Ok(Self { user_service })
    }

    pub async fn find_all(&self, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<User>, String> {
        self.user_service.find_all(limit, offset).await
    }

    pub async fn find_by_role(&self, role: &str) -> Result<Vec<User>, String> {
        self.user_service.find_by_role(role).await
    }

    pub async fn count(&self) -> Result<u64, String> {
        self.user_service.count().await
    }
} 