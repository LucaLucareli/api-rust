use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json as JsonResponse,
};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use crate::services::catalog_service::CatalogService;

#[derive(Debug, Serialize)]
pub struct VideoCatalogResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub duration: u32,
    pub genre: String,
    pub thumbnail_url: String,
}

pub struct CatalogController;

impl CatalogController {
    pub async fn get_videos(
        State(db): State<DatabaseConnection>,
    ) -> Result<JsonResponse<Vec<VideoCatalogResponse>>, StatusCode> {
        let catalog_service = CatalogService::new(db);
        
        match catalog_service.get_videos_with_cache().await {
            Ok(videos) => {
                let response: Vec<VideoCatalogResponse> = videos
                    .into_iter()
                    .map(|video| VideoCatalogResponse {
                        id: video.id,
                        title: video.title,
                        description: video.description,
                        duration: video.duration,
                        genre: video.genre,
                        thumbnail_url: video.thumbnail_url,
                    })
                    .collect();
                Ok(JsonResponse(response))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_video_by_id(
        State(db): State<DatabaseConnection>,
        Path(video_id): Path<String>,
    ) -> Result<JsonResponse<VideoCatalogResponse>, StatusCode> {
        let catalog_service = CatalogService::new(db);
        
        match catalog_service.get_video_by_id(&video_id).await {
            Ok(Some(video)) => {
                let response = VideoCatalogResponse {
                    id: video.id,
                    title: video.title,
                    description: video.description,
                    duration: video.duration,
                    genre: video.genre,
                    thumbnail_url: video.thumbnail_url,
                };
                Ok(JsonResponse(response))
            }
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn health() -> JsonResponse<&'static str> {
        JsonResponse("Viewer API - Healthy")
    }
}
