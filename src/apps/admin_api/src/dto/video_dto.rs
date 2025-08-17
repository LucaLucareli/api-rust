use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct CreateVideoRequestDto {
    pub title: String,
    pub description: String,
    pub duration_seconds: u32,
    pub release_year: Option<u32>,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVideoRequestDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration_seconds: Option<u32>,
    pub release_year: Option<u32>,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VideoResponseDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub duration_seconds: u32,
    pub release_year: Option<u32>,
    pub rating: f32,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
    pub is_featured: bool,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
