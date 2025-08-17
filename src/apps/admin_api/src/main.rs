use axum::Router;
use tracing_subscriber;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Configurar logging automático
    tracing_subscriber::fmt::init();

    // Carregar configuração do .env
    dotenvy::dotenv().ok();
    
    let port = std::env::var("ADMIN_API_PORT")
        .unwrap_or_else(|_| "3002".to_string())
        .parse::<u16>()
        .unwrap_or(3002);

    // Configurar endereço
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Criar router
    let app = Router::new()
        .nest("/admin", admin_api::routes::create_router())
        .route("/", axum::routing::get(|| async { "Admin API - Running" }));

    // Logs automáticos
    tracing::info!("🚀 Admin API iniciando em http://{}", addr);
    tracing::info!("📱 Endpoints disponíveis:");
    tracing::info!("   - POST /admin/videos");
    tracing::info!("   - GET  /admin/videos");
    tracing::info!("   - GET  /admin/videos/:video_id");
    tracing::info!("   - PUT  /admin/videos/:video_id");
    tracing::info!("   - DELETE /admin/videos/:video_id");
    tracing::info!("   - GET  /admin/health");
    tracing::info!("   - GET  /");

    // ✅ Nova forma no Axum 0.8
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("✅ Admin API pronta e rodando!");
    axum::serve(listener, app)
        .await
        .unwrap();
}
