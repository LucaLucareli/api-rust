use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VideoCatalogResponseDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub duration_seconds: u32,
    pub release_year: Option<u32>,
    pub rating: f32,
    pub genre: String,
    pub thumbnail_url: Option<String>,
    pub is_featured: bool,
}
