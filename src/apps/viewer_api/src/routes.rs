use axum::{routing::get, Router};
use crate::controllers::CatalogController;

pub fn create_router() -> Router {
    Router::new()
        .route("/videos", get(CatalogController::get_videos))
        .route("/health", axum::routing::get(CatalogController::health))
}
