use axum::{routing::{post, get}, Router};
use crate::controllers::{
    AuthController,
    // login,
    // register,
    // refresh_token,
    list_users,
    count_users,
    get_users_by_role,
    get_user_by_id,
    get_user_by_email,
    create_user
};
use sea_orm::DatabaseConnection;

pub fn create_router() -> Router<DatabaseConnection> {
    Router::new()
        // Rotas de autenticação
        //TODO: Corrigir
        // .route("/login", post(login))
        // .route("/register", post(register))
        // .route("/refresh", post(refresh_token))
        .route("/health", get(AuthController::health))
        // Rotas de busca de usuários
        .route("/users", get(list_users))
        .route("/users/count", get(count_users))
        .route("/users/by-role", get(get_users_by_role))
        // Rotas de usuário específico
        .route("/users/:user_id", get(get_user_by_id))
        .route("/users/email/:email", get(get_user_by_email))
        // Rotas de criação
        .route("/users", post(create_user))
} 