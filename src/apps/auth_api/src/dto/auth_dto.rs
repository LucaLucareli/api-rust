use serde::{Deserialize, Serialize};
use super::user_dto::UserResponseDto;

#[derive(Debug, Deserialize)]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequestDto {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequestDto {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub user: UserResponseDto,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponseDto {
    pub user: UserResponseDto,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponseDto {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
} 
