use sea_orm::DatabaseConnection;
use crate::models::user::User;
use crate::services::user_service::UserService;

pub struct FindUserByIdService {
    user_service: UserService,
}

impl FindUserByIdService {
    pub fn new(db: DatabaseConnection, redis_url: String) -> Result<Self, redis::RedisError> {
        let user_service = UserService::new(db, redis_url)?;
        Ok(Self { user_service })
    }

    pub async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, String> {
        self.user_service.get_user_by_id(user_id).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        self.user_service.get_user_by_email(email).await
    }
} 