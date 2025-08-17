use axum::{
    http::StatusCode,
    response::Json as JsonResponse,
};
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

pub struct CatalogController {
    catalog_service: CatalogService,
}

impl CatalogController {
    pub fn new() -> Self {
        Self {
            catalog_service: CatalogService::new(),
        }
    }

    pub async fn get_videos() -> Result<JsonResponse<Vec<VideoCatalogResponse>>, StatusCode> {
        let controller = Self::new();
        
        match controller.catalog_service.get_videos_with_cache().await {
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

    pub async fn health() -> &'static str {
        "Viewer API - Healthy"
    }
}
