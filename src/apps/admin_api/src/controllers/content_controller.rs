use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as JsonResponse,
};
use serde::{Deserialize, Serialize};
use crate::services::content_service::ContentService;

#[derive(Debug, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub description: String,
    pub duration: u32,
    pub genre: String,
}

#[derive(Debug, Serialize)]
pub struct VideoResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub duration: u32,
    pub genre: String,
}

pub struct ContentController {
    content_service: ContentService,
}

impl ContentController {
    pub fn new() -> Self {
        Self {
            content_service: ContentService::new(),
        }
    }

    pub async fn create_video(
        Json(payload): Json<CreateVideoRequest>,
    ) -> Result<JsonResponse<VideoResponse>, StatusCode> {
        let controller = Self::new();
        
        match controller.content_service.create_video(&payload).await {
            Ok(video) => {
                let response = VideoResponse {
                    id: video.id,
                    title: video.title,
                    description: video.description,
                    duration: video.duration,
                    genre: video.genre,
                };
                Ok(JsonResponse(response))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn health() -> &'static str {
        "Admin API - Healthy"
    }
}
