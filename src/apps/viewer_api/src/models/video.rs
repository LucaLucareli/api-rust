use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
    pub duration: u32,
    pub genre: String,
    pub thumbnail_url: String,
}
