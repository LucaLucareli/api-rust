use axum::{routing::{post, get, put, delete}, Router};
use sea_orm::DatabaseConnection;

pub fn create_router() -> Router<DatabaseConnection> {
    Router::new()
        // Rotas de vídeo
        .route("/videos", post(crate::controllers::video_controller::create_video))
        .route("/videos", get(crate::controllers::video_controller::list_videos))
        .route("/videos/:video_id", get(crate::controllers::video_controller::get_video_by_id))
        .route("/videos/:video_id", put(crate::controllers::video_controller::update_video))
        .route("/videos/:video_id", delete(crate::controllers::video_controller::delete_video))
        
        // Health check
        .route("/health", get(crate::controllers::video_controller::health))
        .route("/", get(|| async { "Admin API - Running" }))
}
