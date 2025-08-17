pub mod config;
pub mod errors;
pub mod apps;
pub mod infrastructure;

// Re-export de módulos comuns
pub use config::Config;
pub use errors::{AppError, Result as AppResult};
