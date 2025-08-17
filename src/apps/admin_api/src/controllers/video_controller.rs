use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::Json as JsonResponse,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use crate::services::video_service::VideoService;
use crate::dto::video_dto::*;

#[derive(Debug, Deserialize)]
pub struct ListVideosQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ListVideosResponseDto {
    pub videos: Vec<VideoResponseDto>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
}

pub struct VideoController;

impl VideoController {
    pub async fn create_video(
        State(db): State<DatabaseConnection>,
        Json(payload): Json<CreateVideoRequestDto>,
    ) -> Result<JsonResponse<VideoResponseDto>, StatusCode> {
        let video_service = VideoService::new(db);
        
        match video_service.create_video(payload).await {
            Ok(video) => Ok(JsonResponse(video)),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    pub async fn get_video_by_id(
        State(db): State<DatabaseConnection>,
        Path(video_id): Path<String>,
    ) -> Result<JsonResponse<VideoResponseDto>, StatusCode> {
        let video_service = VideoService::new(db);
        
        match video_service.get_video_by_id(&video_id).await {
            Ok(Some(video)) => Ok(JsonResponse(video)),
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn update_video(
        State(db): State<DatabaseConnection>,
        Path(video_id): Path<String>,
        Json(payload): Json<UpdateVideoRequestDto>,
    ) -> Result<JsonResponse<VideoResponseDto>, StatusCode> {
        let video_service = VideoService::new(db);
        
        match video_service.update_video(&video_id, payload).await {
            Ok(Some(video)) => Ok(JsonResponse(video)),
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn delete_video(
        State(db): State<DatabaseConnection>,
        Path(video_id): Path<String>,
    ) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
        let video_service = VideoService::new(db);
        
        match video_service.delete_video(&video_id).await {
            Ok(true) => {
                let response = serde_json::json!({
                    "message": "Vídeo deletado com sucesso"
                });
                Ok(JsonResponse(response))
            }
            Ok(false) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn list_videos(
        State(db): State<DatabaseConnection>,
        Query(query): Query<ListVideosQuery>,
    ) -> Result<JsonResponse<ListVideosResponseDto>, StatusCode> {
        let video_service = VideoService::new(db);
        
        match video_service.list_videos(query.limit, query.offset).await {
            Ok(videos) => {
                let response = ListVideosResponseDto {
                    videos: videos.clone(),
                    total: videos.len() as u64,
                    limit: query.limit.unwrap_or(10),
                    offset: query.offset.unwrap_or(0),
                };
                Ok(JsonResponse(response))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn health() -> JsonResponse<&'static str> {
        JsonResponse("Admin API - Healthy")
    }
}
