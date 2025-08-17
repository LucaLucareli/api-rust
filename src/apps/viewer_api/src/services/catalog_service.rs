use sea_orm::DatabaseConnection;
use crate::models::video::Video;
use std::time::Duration;

// Importar o repositório de vídeos
use api_rust::libs::shared::database::repositories::videos::{VideosRepository, Video as RepoVideo};

pub struct CatalogService {
    db: DatabaseConnection,
    cache_ttl: Duration,
}

impl CatalogService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            cache_ttl: Duration::from_secs(300), // 5 minutos
        }
    }

    pub async fn get_videos(&self) -> Result<Vec<Video>, String> {
        let videos_repo = VideosRepository::new(self.db.clone());
        
        match videos_repo.find_all(None, None).await {
            Ok(repo_videos) => {
                // Converter do repositório para o modelo da API
                let api_videos: Vec<Video> = repo_videos.into_iter().map(|repo_video| Video {
                    id: repo_video.id,
                    title: repo_video.title,
                    description: repo_video.description,
                    duration: repo_video.duration_seconds as u32,
                    genre: "Geral".to_string(), // TODO: Adicionar campo genre no repositório
                    thumbnail_url: repo_video.thumbnail_url.unwrap_or_else(|| "".to_string()),
                }).collect();
                
                Ok(api_videos)
            }
            Err(e) => Err(format!("Erro ao buscar vídeos: {}", e)),
        }
    }

    pub async fn get_videos_with_cache(&self) -> Result<Vec<Video>, String> {
        let cache_key = "videos:catalog";
        
        // TODO: Implementar cache real com Redis
        // Por enquanto, sempre busca do banco
        println!("🔍 Verificando cache para chave: {}", cache_key);
        println!("💾 Cache MISS - buscando do banco de dados");
        
        let videos = self.get_videos().await?;
        
        // TODO: Salvar no cache Redis
        println!("💾 Salvando resultado no cache");
        
        Ok(videos)
    }

    pub async fn get_video_by_id(&self, video_id: &str) -> Result<Option<Video>, String> {
        let videos_repo = VideosRepository::new(self.db.clone());
        
        match videos_repo.find_by_id(video_id).await {
            Ok(Some(repo_video)) => {
                // Converter do repositório para o modelo da API
                let api_video = Video {
                    id: repo_video.id,
                    title: repo_video.title,
                    description: repo_video.description,
                    duration: repo_video.duration_seconds as u32,
                    genre: "Geral".to_string(), // TODO: Adicionar campo genre no repositório
                    thumbnail_url: repo_video.thumbnail_url.unwrap_or_else(|| "".to_string()),
                };
                
                Ok(Some(api_video))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Erro ao buscar vídeo: {}", e)),
        }
    }
}
