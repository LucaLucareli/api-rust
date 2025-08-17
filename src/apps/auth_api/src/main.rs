use axum::Router;
use tracing_subscriber;
use std::net::SocketAddr;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Configurar logging automático
    tracing_subscriber::fmt::init();

    // Carregar configuração do .env
    dotenvy::dotenv().ok();
    
    let port = std::env::var("AUTH_API_PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);

    // Configurar conexão com banco de dados
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlserver://localhost:1433/streaming_db".to_string());
    
    let db: DatabaseConnection = Database::connect(&database_url).await
        .expect("Falha ao conectar com banco de dados");

    // Configurar endereço
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Criar router com state
    let app = Router::new()
        .nest("/auth", auth_api::routes::create_router())
        .route("/", axum::routing::get(|| async { "Auth API - Running" }))
        .with_state(db);

    // Logs automáticos
    tracing::info!("🚀 Auth API iniciando em http://{}", addr);
    tracing::info!("📱 Endpoints disponíveis:");
    tracing::info!("   - POST /auth/login");
    tracing::info!("   - POST /auth/register");
    tracing::info!("   - POST /auth/refresh");
    tracing::info!("   - GET  /auth/health");
    tracing::info!("   - GET  /auth/users/:user_id");
    tracing::info!("   - PUT  /auth/users/:user_id");
    tracing::info!("   - DELETE /auth/users/:user_id");
    tracing::info!("   - GET  /");

    // ✅ Nova forma no Axum 0.8
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("✅ Auth API pronta e rodando!");
    axum::serve(listener, app)
        .await
        .unwrap();
}
