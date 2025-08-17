pub mod config;
pub mod errors;
pub mod infrastructure;
pub mod apps;
pub mod libs;

pub use config::Config;
pub use errors::Result;
pub use libs::modules::{AuthService, Claims, User, LoginRequest, RegisterRequest, AuthResponse, UserInfo};
pub use apps::{create_auth_router, create_admin_router, create_viewer_router};
