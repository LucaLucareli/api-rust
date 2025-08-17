use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::dto::UserResponseDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: String, email: String, name: String, role: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            email,
            name,
            role,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            role: crate::dto::UserRoleDto::from(user.role.as_str()),
        }
    }
}
