use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserResponseDto {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: UserRoleDto,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRoleDto {
    Admin,
    Viewer,
}

impl From<&str> for UserRoleDto {
    fn from(role: &str) -> Self {
        match role.to_lowercase().as_str() {
            "admin" => UserRoleDto::Admin,
            "viewer" => UserRoleDto::Viewer,
            _ => UserRoleDto::Viewer,
        }
    }
}

impl ToString for UserRoleDto {
    fn to_string(&self) -> String {
        match self {
            UserRoleDto::Admin => "admin".to_string(),
            UserRoleDto::Viewer => "viewer".to_string(),
        }
    }
}
