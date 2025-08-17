//https://www.tabnews.com.br/ddanielsantos/criando-uma-api-rest-com-rust

use std::net::ToSocketAddrs;
use axum::{routing::get, Router};
use hyper::Request;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::tokio::TokioIo;
use tokio::net::TcpListener;
use tower_service::Service;
use tracing_subscriber;
use api_rust::config::Config;

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

    let listener = TcpListener::bind(addr).await.unwrap();

    // Router principal que combina todas as APIs
    let app = Router::new()
        .route("/", get(|| async { "API Rust Monorepo - Status: OK" }))
        .route("/health", get(|| async { "Healthy" }))
        .route("/auth/{*path}", get(|| async { "Auth API - Em desenvolvimento" }))
        .route("/admin/{*path}", get(|| async { "Admin API - Em desenvolvimento" }))
        .route("/viewer/{*path}", get(|| async { "Viewer API - Em desenvolvimento" }));

    let make_service = app.into_make_service();

    // Logs estilo NestJS
    println!("🚀 API Rust Monorepo iniciando...");
    println!("📊 Configuração carregada:");
    println!("   - API Principal: http://localhost:{}", config.api_port);
    println!("   - Auth API: http://localhost:{}", config.auth_api_port);
    println!("   - Admin API: http://localhost:{}", config.admin_api_port);
    println!("   - Viewer API: http://localhost:{}", config.viewer_api_port);
    println!("   - Log Level: {}", config.log_level);
    println!("");
    println!("🌐 Serviços disponíveis:");
    println!("   - Auth API: http://localhost:{}/auth", config.auth_api_port);
    println!("   - Admin API: http://localhost:{}/admin", config.admin_api_port);
    println!("   - Viewer API: http://localhost:{}/viewer", config.viewer_api_port);
    println!("");
    println!("✅ API Rust Monorepo iniciada com sucesso!");
    println!("🎯 Aguardando conexões em http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);
        let mut make_service = make_service.clone();

        tokio::spawn(async move {
            // Cria o serviço para esta conexão
            let service = make_service.call(()).await.unwrap();

            // Usa `service_fn` para servir as requisições
            let handler = service_fn(move |req: Request<_>| {
                let mut svc = service.clone();
                async move {
                    // Infallible porque Axum não falha
                    svc.call(req).await.map_err(|_err| std::io::Error::new(std::io::ErrorKind::Other, "handler failed"))
                }
            });

            if let Err(err) = http1::Builder::new()
                .serve_connection(io, handler)
                .await
            {
                eprintln!("server error: {}", err);
            }
        });
    }
}
