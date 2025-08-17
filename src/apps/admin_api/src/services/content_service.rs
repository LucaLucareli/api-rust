use crate::models::video::Video;
use crate::controllers::CreateVideoRequest;

pub struct ContentService;

impl ContentService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_video(&self, request: &CreateVideoRequest) -> Result<Video, String> {
        // Simulação de criação de vídeo
        let video = Video {
            id: "video_1".to_string(),
            title: request.title.clone(),
            description: request.description.clone(),
            duration: request.duration,
            genre: request.genre.clone(),
        };
        Ok(video)
    }
}
