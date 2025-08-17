use sea_orm::DatabaseConnection;
use crate::dto::video_dto::{CreateVideoRequestDto, UpdateVideoRequestDto, VideoResponseDto};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Importar o repositório de vídeos
use api_rust::libs::shared::database::repositories::videos::{VideosRepository, CreateVideoRequest, UpdateVideoRequest, Video as RepoVideo};

pub struct VideoService {
    db: DatabaseConnection,
}

impl VideoService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_video(&self, request: CreateVideoRequestDto) -> Result<VideoResponseDto, String> {
        let videos_repo = VideosRepository::new(self.db.clone());
        
        // Converter DTO da API para DTO do repositório
        let repo_request = CreateVideoRequest {
            title: request.title,
            description: request.description,
            duration_seconds: request.duration_seconds,
            release_year: request.release_year,
            thumbnail_url: request.thumbnail_url,
            video_url: request.video_url,
            trailer_url: request.trailer_url,
        };
        
        match videos_repo.create(repo_request).await {
            Ok(repo_video) => {
                // Converter do repositório para DTO da API
                let api_video = VideoResponseDto {
                    id: repo_video.id,
                    title: repo_video.title,
                    description: repo_video.description,
                    duration_seconds: repo_video.duration_seconds,
                    release_year: repo_video.release_year,
                    rating: repo_video.rating,
                    thumbnail_url: repo_video.thumbnail_url,
                    video_url: repo_video.video_url,
                    trailer_url: repo_video.trailer_url,
                    is_featured: repo_video.is_featured,
                    is_available: repo_video.is_available,
                    created_at: repo_video.created_at,
                    updated_at: repo_video.updated_at,
                };
                
                Ok(api_video)
            }
            Err(e) => Err(format!("Erro ao criar vídeo: {}", e)),
        }
    }

    pub async fn get_video_by_id(&self, video_id: &str) -> Result<Option<VideoResponseDto>, String> {
        let videos_repo = VideosRepository::new(self.db.clone());
        
        match videos_repo.find_by_id(video_id).await {
            Ok(Some(repo_video)) => {
                // Converter do repositório para DTO da API
                let api_video = VideoResponseDto {
                    id: repo_video.id,
                    title: repo_video.title,
                    description: repo_video.description,
                    duration_seconds: repo_video.duration_seconds,
                    release_year: repo_video.release_year,
                    rating: repo_video.rating,
                    thumbnail_url: repo_video.thumbnail_url,
                    video_url: repo_video.video_url,
                    trailer_url: repo_video.trailer_url,
                    is_featured: repo_video.is_featured,
                    is_available: repo_video.is_available,
                    created_at: repo_video.created_at,
                    updated_at: repo_video.updated_at,
                };
                
                Ok(Some(api_video))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Erro ao buscar vídeo: {}", e)),
        }
    }

    pub async fn update_video(&self, video_id: &str, request: UpdateVideoRequestDto) -> Result<Option<VideoResponseDto>, String> {
        let videos_repo = VideosRepository::new(self.db.clone());
        
        // Converter DTO da API para DTO do repositório
        let repo_request = UpdateVideoRequest {
            title: request.title,
            description: request.description,
            duration_seconds: request.duration_seconds,
            release_year: request.release_year,
            rating: request.rating,
            thumbnail_url: request.thumbnail_url,
            video_url: request.video_url,
            trailer_url: request.trailer_url,
            is_featured: request.is_featured,
            is_available: request.is_available,
        };
        
        match videos_repo.update(video_id, repo_request).await {
            Ok(Some(repo_video)) => {
                // Converter do repositório para DTO da API
                let api_video = VideoResponseDto {
                    id: repo_video.id,
                    title: repo_video.title,
                    description: repo_video.description,
                    duration_seconds: repo_video.duration_seconds,
                    release_year: repo_video.release_year,
                    rating: repo_video.rating,
                    thumbnail_url: repo_video.thumbnail_url,
                    video_url: repo_video.video_url,
                    trailer_url: repo_video.trailer_url,
                    is_featured: repo_video.is_featured,
                    is_available: repo_video.is_available,
                    created_at: repo_video.created_at,
                    updated_at: repo_video.updated_at,
                };
                
                Ok(Some(api_video))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Erro ao atualizar vídeo: {}", e)),
        }
    }

    pub async fn delete_video(&self, video_id: &str) -> Result<bool, String> {
        let videos_repo = VideosRepository::new(self.db.clone());
        
        match videos_repo.delete(video_id).await {
            Ok(success) => Ok(success),
            Err(e) => Err(format!("Erro ao deletar vídeo: {}", e)),
        }
    }

    pub async fn list_videos(&self, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<VideoResponseDto>, String> {
        let videos_repo = VideosRepository::new(self.db.clone());
        
        match videos_repo.find_all(limit, offset).await {
            Ok(repo_videos) => {
                // Converter do repositório para DTOs da API
                let api_videos: Vec<VideoResponseDto> = repo_videos.into_iter().map(|repo_video| VideoResponseDto {
                    id: repo_video.id,
                    title: repo_video.title,
                    description: repo_video.description,
                    duration_seconds: repo_video.duration_seconds,
                    release_year: repo_video.release_year,
                    rating: repo_video.rating,
                    thumbnail_url: repo_video.thumbnail_url,
                    video_url: repo_video.video_url,
                    trailer_url: repo_video.trailer_url,
                    is_featured: repo_video.is_featured,
                    is_available: repo_video.is_available,
                    created_at: repo_video.created_at,
                    updated_at: repo_video.updated_at,
                }).collect();
                
                Ok(api_videos)
            }
            Err(e) => Err(format!("Erro ao listar vídeos: {}", e)),
        }
    }
}
