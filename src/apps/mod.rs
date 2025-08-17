// Módulo principal para todas as APIs do workspace
// As APIs são crates separados e não módulos deste crate

pub mod auth_api;
pub mod admin_api;
pub mod viewer_api;

// Re-export das funções de roteamento das APIs
pub use auth_api::routes::create_router as create_auth_router;
pub use admin_api::routes::create_router as create_admin_router;
pub use viewer_api::routes::create_router as create_viewer_router;
