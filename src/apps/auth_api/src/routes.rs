use axum::{routing::{post, get, put, delete}, Router};
use sea_orm::DatabaseConnection;

pub fn create_router() -> Router<DatabaseConnection> {
    Router::new()
        // Rotas de autenticação
        .route("/login", post(crate::controllers::auth_controller::login))
        .route("/register", post(crate::controllers::auth_controller::register))
        .route("/refresh", post(crate::controllers::auth_controller::refresh_token))
        .route("/health", get(crate::controllers::auth_controller::health))
        
        // Rotas de usuários (protegidas)
        .route("/users", get(crate::controllers::user_controller::list_users))
        .route("/users/count", get(crate::controllers::user_controller::count_users))
        .route("/users/by-role", get(crate::controllers::user_controller::get_users_by_role))
        .route("/users/:user_id", get(crate::controllers::user_controller::get_user_by_id))
        .route("/users/email/:email", get(crate::controllers::user_controller::get_user_by_email))
        .route("/users", post(crate::controllers::user_controller::create_user))
        .route("/users/:user_id", put(crate::controllers::user_controller::update_user))
        .route("/users/:user_id", delete(crate::controllers::user_controller::delete_user))
} 