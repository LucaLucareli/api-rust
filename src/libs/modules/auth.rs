use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: i64,    // expiration
    pub iat: i64,    // issued at
    pub email: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub role: String,
}

pub struct AuthService {
    access_secret: String,
    refresh_secret: String,
    access_expiry_hours: u64,
    refresh_expiry_days: u64,
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl AuthService {
    pub fn new(
        access_secret: String,
        refresh_secret: String,
        access_expiry_hours: u64,
        refresh_expiry_days: u64,
    ) -> Self {
        Self {
            access_secret,
            refresh_secret,
            access_expiry_hours,
            refresh_expiry_days,
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<UserInfo, String> {
        let mut users = self.users.write().await;
        
        // Verificar se o usuário já existe
        if users.values().any(|u| u.email == req.email) {
            return Err("Usuário já existe".to_string());
        }

        let user_id = Uuid::new_v4().to_string();
        let password_hash = hash(req.password.as_bytes(), DEFAULT_COST)
            .map_err(|_| "Erro ao criptografar senha")?;
        
        let now = Utc::now().timestamp();
        let role = req.role.unwrap_or_else(|| "user".to_string());

        let user = User {
            id: user_id.clone(),
            email: req.email,
            password_hash,
            role,
            created_at: now,
            updated_at: now,
        };

        users.insert(user_id.clone(), user);

        Ok(UserInfo {
            id: user_id,
            email: req.email,
            role: role,
        })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, String> {
        let users = self.users.read().await;
        
        let user = users
            .values()
            .find(|u| u.email == req.email)
            .ok_or("Usuário não encontrado")?;

        if !verify(req.password.as_bytes(), &user.password_hash)
            .map_err(|_| "Erro ao verificar senha")? {
            return Err("Senha incorreta".to_string());
        }

        let access_token = self.generate_access_token(user)?;
        let refresh_token = self.generate_refresh_token(user)?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user: UserInfo {
                id: user.id.clone(),
                email: user.email.clone(),
                role: user.role.clone(),
            },
        })
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, String> {
        let key = DecodingKey::from_secret(self.access_secret.as_ref());
        let token_data = decode::<Claims>(token, &key, &Validation::default())
            .map_err(|_| "Token inválido")?;

        Ok(token_data.claims)
    }

    pub fn refresh_token(&self, refresh_token: &str) -> Result<String, String> {
        let key = DecodingKey::from_secret(self.refresh_secret.as_ref());
        let token_data = decode::<Claims>(refresh_token, &key, &Validation::default())
            .map_err(|_| "Refresh token inválido")?;

        // Gerar novo access token
        let user_id = token_data.claims.sub;
        let users = tokio::runtime::Handle::current().block_on(async {
            let users = self.users.read().await;
            users.get(&user_id).cloned()
        });

        if let Some(user) = users {
            self.generate_access_token(&user)
        } else {
            Err("Usuário não encontrado".to_string())
        }
    }

    fn generate_access_token(&self, user: &User) -> Result<String, String> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.access_expiry_hours as i64);

        let claims = Claims {
            sub: user.id.clone(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            email: user.email.clone(),
            role: user.role.clone(),
        };

        let key = EncodingKey::from_secret(self.access_secret.as_ref());
        encode(&Header::default(), &claims, &key)
            .map_err(|_| "Erro ao gerar token".to_string())
    }

    fn generate_refresh_token(&self, user: &User) -> Result<String, String> {
        let now = Utc::now();
        let expires_at = now + Duration::days(self.refresh_expiry_days as i64);

        let claims = Claims {
            sub: user.id.clone(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            email: user.email.clone(),
            role: user.role.clone(),
        };

        let key = EncodingKey::from_secret(self.refresh_secret.as_ref());
        encode(&Header::default(), &claims, &key)
            .map_err(|_| "Erro ao gerar refresh token".to_string())
    }

    // Método para inicializar com alguns usuários de teste
    pub async fn initialize_test_users(&self) {
        let mut users = self.users.write().await;
        
        let admin_password = hash("admin123".as_bytes(), DEFAULT_COST).unwrap();
        let user_password = hash("user123".as_bytes(), DEFAULT_COST).unwrap();
        
        let now = Utc::now().timestamp();
        
        users.insert(
            "admin-001".to_string(),
            User {
                id: "admin-001".to_string(),
                email: "admin@example.com".to_string(),
                password_hash: admin_password,
                role: "admin".to_string(),
                created_at: now,
                updated_at: now,
            },
        );
        
        users.insert(
            "user-001".to_string(),
            User {
                id: "user-001".to_string(),
                email: "user@example.com".to_string(),
                password_hash: user_password,
                role: "user".to_string(),
                created_at: now,
                updated_at: now,
            },
        );
    }
}

// Middleware de autenticação
pub async fn auth_middleware(
    jar: CookieJar,
    State(auth_service): State<Arc<AuthService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token_from_request(&jar, &request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = auth_service
        .validate_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Adicionar claims ao request para uso posterior
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}

// Middleware de autenticação com verificação de role
pub async fn auth_middleware_with_role(
    required_role: &'static str,
    jar: CookieJar,
    State(auth_service): State<Arc<AuthService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token_from_request(&jar, &request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = auth_service
        .validate_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Verificar se o usuário tem a role necessária
    if claims.role != required_role && claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Adicionar claims ao request para uso posterior
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}

// Função auxiliar para extrair token do request
fn extract_token_from_request(jar: &CookieJar, request: &Request) -> Option<String> {
    // Tentar extrair do header Authorization
    if let Some(auth_header) = request.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }

    // Tentar extrair do cookie
    if let Some(token_cookie) = jar.get("access_token") {
        return Some(token_cookie.value().to_string());
    }

    None
}

// Função para extrair claims do request (para uso nos handlers)
pub fn extract_claims(request: &Request) -> Option<&Claims> {
    request.extensions().get::<Claims>()
}
