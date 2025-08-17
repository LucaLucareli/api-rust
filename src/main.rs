//https://www.tabnews.com.br/ddanielsantos/criando-uma-api-rest-com-rust

use std::net::ToSocketAddrs;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing_subscriber;
use std::sync::Arc;

use api_rust::{
    config::Config,
    apps::{create_auth_router, create_admin_router, create_viewer_router},
    libs::modules::AuthService,
    infrastructure::db::DatabaseConnection,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    tracing_subscriber::fmt::init();

    // Carregar configuração
    let config = Config::from_env()?;

    let addr = format!("localhost:{}", config.api_port)
        .to_socket_addrs()?
        .next()
        .expect("Erro ao resolver localhost");

    // Inicializar serviços
    let auth_service = Arc::new(AuthService::new(
        config.jwt_access_secret.clone(),
        config.jwt_refresh_secret.clone(),
        config.jwt_access_expiry_hours,
        config.jwt_refresh_expiry_days,
    ));

    // Inicializar usuários de teste
    auth_service.initialize_test_users().await;

    // Conectar ao banco de dados (em produção, usar conexão real)
    let db = DatabaseConnection::new(&config.database_url).await?;

    // Criar router principal que integra automaticamente todas as APIs
    let app = Router::new()
        // Rotas públicas
        .route("/", get(|| async { "API Rust Monorepo - Status: OK" }))
        .route("/health", get(|| async { "Healthy" }))
        
        // Integrar automaticamente as APIs da pasta apps
        .nest("/auth", create_auth_router())
        .nest("/admin", create_admin_router())
        .nest("/viewer", create_viewer_router())
        
        .with_state(db);

    let listener = TcpListener::bind(addr).await.unwrap();

    // Logs estilo NestJS
    println!("🚀 API Rust Monorepo iniciando...");
    println!("📊 Configuração carregada:");
    println!("   - API Principal: http://localhost:{}", config.api_port);
    println!("   - Log Level: {}", config.log_level);
    println!("");
    println!("🌐 APIs integradas automaticamente:");
    println!("   - Auth API: http://localhost:{}/auth", config.api_port);
    println!("   - Admin API: http://localhost:{}/admin", config.api_port);
    println!("   - Viewer API: http://localhost:{}/viewer", config.api_port);
    println!("");
    println!("🔐 Usuários de teste:");
    println!("   - Admin: admin@example.com / admin123");
    println!("   - User: user@example.com / user123");
    println!("");
    println!("✅ API Rust Monorepo iniciada com sucesso!");
    println!("🎯 Aguardando conexões em http://{}", addr);

    // Servir a aplicação
    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())
}
