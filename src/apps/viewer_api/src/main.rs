use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Configurar logging automático
    tracing_subscriber::fmt::init();

    // Criar router
    let app = Router::new()
        .nest("/viewer", viewer_api::routes::create_router())
        .route("/", axum::routing::get(|| async { "Viewer API - Running" }));

    // Configurar endereço
    let addr = SocketAddr::from(([127, 0, 0, 1], 3003));
    
    // Logs automáticos
    tracing::info!("🚀 Viewer API iniciando em http://{}", addr);
    tracing::info!("👁️  Endpoints disponíveis:");
    tracing::info!("   - GET  /viewer/videos (com cache)");
    tracing::info!("   - GET  /viewer/health");
    tracing::info!("   - GET  /");

    // ✅ Nova forma no Axum 0.8
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("✅ Viewer API pronta e rodando!");
    axum::serve(listener, app)
        .await
        .unwrap();
}
