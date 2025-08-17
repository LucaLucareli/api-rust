// use axum::{routing::{post, get, put, delete}, Router};
use axum::{Router};
// use crate::controllers::VideoController;

pub fn create_router() -> Router {
    Router::new()
        // Rotas de vídeo
        // .route("/videos", post(VideoController::create_video))
        // .route("/videos", get(VideoController::list_videos))
        // .route("/videos/:video_id", get(VideoController::get_video_by_id))
        // .route("/videos/:video_id", put(VideoController::update_video))
        // .route("/videos/:video_id", delete(VideoController::delete_video))
        // Health check
        .route("/health", axum::routing::get(|| async { "Admin API - Healthy" }))
        .route("/", axum::routing::get(|| async { "Admin API - Running" }))
}
