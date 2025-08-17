use crate::models::video::Video;
use std::time::Duration;

pub struct CatalogService {
    cache_ttl: Duration,
}

impl CatalogService {
    pub fn new() -> Self {
        Self {
            cache_ttl: Duration::from_secs(300), // 5 minutos
        }
    }

    pub async fn get_videos(&self) -> Result<Vec<Video>, String> {
        // Simulação de busca com cache
        let cache_key = "videos:catalog";
        
        // TODO: Implementar cache real
        // Primeiro tenta buscar do cache
        // Se não encontrar, busca do banco e salva no cache
        
        let videos = vec![
            Video {
                id: "video_1".to_string(),
                title: "Aventuras Espaciais".to_string(),
                description: "Uma jornada épica pelo universo".to_string(),
                duration: 120,
                genre: "Aventura".to_string(),
                thumbnail_url: "https://example.com/thumb1.jpg".to_string(),
            },
            Video {
                id: "video_2".to_string(),
                title: "Mistério na Cidade".to_string(),
                description: "Um thriller cheio de suspense".to_string(),
                duration: 95,
                genre: "Suspense".to_string(),
                thumbnail_url: "https://example.com/thumb2.jpg".to_string(),
            },
        ];
        
        // TODO: Salvar no cache
        println!("💾 Salvando vídeos no cache com TTL: {:?}", self.cache_ttl);
        
        Ok(videos)
    }

    pub async fn get_videos_with_cache(&self) -> Result<Vec<Video>, String> {
        let cache_key = "videos:catalog";
        
        // Simulação de verificação de cache
        println!("🔍 Verificando cache para chave: {}", cache_key);
        
        // Simulação de cache miss
        println!("💾 Cache MISS - buscando do banco de dados");
        
        let videos = self.get_videos().await?;
        
        // Simulação de salvamento no cache
        println!("💾 Salvando resultado no cache");
        
        Ok(videos)
    }
}
