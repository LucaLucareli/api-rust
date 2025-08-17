use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

pub fn create_router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/videos", get(crate::controllers::catalog_controller::get_videos))
        .route("/videos/:video_id", get(crate::controllers::catalog_controller::get_video_by_id))
        .route("/health", get(crate::controllers::catalog_controller::health))
        .route("/", get(|| async { "Viewer API - Running" }))
}
