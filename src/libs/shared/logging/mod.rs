use axum::{
    extract::Request,
    http::{Method, StatusCode, Uri},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, warn, error, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct LoggingMiddleware;

impl LoggingMiddleware {
    pub fn init() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    pub async fn log_request(
        method: Method,
        uri: Uri,
        status: StatusCode,
        latency: std::time::Duration,
        user_agent: Option<String>,
    ) {
        let level = if status.is_success() {
            Level::INFO
        } else if status.is_client_error() {
            Level::WARN
        } else {
            Level::ERROR
        };

        let message = format!(
            "{} {} {} - {}ms - User-Agent: {}",
            method,
            uri,
            status,
            latency.as_millis(),
            user_agent.unwrap_or_else(|| "Unknown".to_string())
        );

        match level {
            Level::INFO => info!("{}", message),
            Level::WARN => warn!("{}", message),
            Level::ERROR => error!("{}", message),
            _ => info!("{}", message),
        }
    }

    pub async fn http_logger(
        request: Request,
        next: Next,
    ) -> Response {
        let start = Instant::now();
        let method = request.method().clone();
        let uri = request.uri().clone();
        let user_agent = request
            .headers()
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        // Log request
        info!(
            "Request: {} {} - User-Agent: {}",
            method,
            uri,
            user_agent.as_deref().unwrap_or("Unknown")
        );

        let response = next.run(request).await;
        let status = response.status();
        let latency = start.elapsed();

        // Log response
        Self::log_request(method, uri, status, latency, user_agent).await;

        response
    }
}

pub fn log_api_start(api_name: &str, port: u16) {
    info!("🚀 {} iniciando na porta {}", api_name, port);
}

pub fn log_api_ready(api_name: &str, port: u16) {
    info!("✅ {} pronta e rodando em http://localhost:{}", api_name, port);
}

pub fn log_endpoint_registered(method: &str, path: &str) {
    info!("📝 Endpoint registrado: {} {}", method, path);
}

pub fn log_database_connection_success() {
    info!("🗄️  Conexão com banco de dados estabelecida com sucesso");
}

pub fn log_database_connection_error(error: &str) {
    error!("❌ Erro na conexão com banco de dados: {}", error);
}

pub fn log_cache_hit(key: &str) {
    info!("💾 Cache HIT: {}", key);
}

pub fn log_cache_miss(key: &str) {
    info!("💾 Cache MISS: {}", key);
}

pub fn log_authentication_success(user_id: &str) {
    info!("🔐 Autenticação bem-sucedida para usuário: {}", user_id);
}

pub fn log_authentication_failure(email: &str, reason: &str) {
    warn!("🔐 Falha na autenticação para {}: {}", email, reason);
}

pub fn log_authorization_failure(user_id: &str, resource: &str) {
    warn!("🚫 Acesso negado para usuário {} ao recurso: {}", user_id, resource);
}
