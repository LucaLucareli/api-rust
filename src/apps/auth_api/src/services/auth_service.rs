use sea_orm::DatabaseConnection;
use crate::dto::TokenPair;
use crate::services::user_service::UserService;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Duration, Utc};
use uuid::Uuid;

pub struct AuthService {
    user_service: UserService,
}

impl AuthService {
    pub fn new(db: DatabaseConnection, jwt_secret: String) -> Result<Self, jsonwebtoken::errors::Error> {
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());
        
        let user_service = UserService::new(db, redis_url)
            .map_err(|e| jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken
            ))?;
        
        Ok(Self {
            user_service,
        })
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<TokenPair, String> {
        // Por enquanto, autenticação mock
        // TODO: Implementar autenticação real
        if email == "admin@example.com" && password == "password" {
            let token_pair = self.generate_token_pair("mock-user-id", email)
                .map_err(|e| format!("Erro ao gerar tokens: {}", e))?;

            Ok(token_pair)
        } else {
            Err("Credenciais inválidas".to_string())
        }
    }

    pub async fn register(&self, email: &str, password: &str, name: &str, role: Option<String>) -> Result<TokenPair, String> {
        // Por enquanto, registro mock
        // TODO: Implementar registro real
        let token_pair = self.generate_token_pair("new-user-id", email)
            .map_err(|e| format!("Erro ao gerar tokens: {}", e))?;

        Ok(token_pair)
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenPair, String> {
        // Por enquanto, refresh mock
        // TODO: Implementar refresh real
        let claims = self.validate_refresh_token(refresh_token)
            .map_err(|e| format!("Token de refresh inválido: {}", e))?;

        let token_pair = self.generate_token_pair(&claims.sub, &claims.email)
            .map_err(|e| format!("Erro ao gerar tokens: {}", e))?;

        Ok(token_pair)
    }

    fn generate_token_pair(&self, user_id: &str, email: &str) -> Result<TokenPair, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let access_exp = now + Duration::hours(1); // 1 hora
        let refresh_exp = now + Duration::days(7);  // 7 dias

        let jwt_secret = std::env::var("JWT_ACCESS_SECRET")
            .unwrap_or_else(|_| "default_secret".to_string());

        // Access Token
        let access_claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: access_exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "access".to_string(),
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )?;

        // Refresh Token
        let refresh_claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: refresh_exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
            token_type: "refresh".to_string(),
        };

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: access_exp.timestamp(),
            refresh_expires_in: refresh_exp.timestamp(),
        })
    }

    fn validate_refresh_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let jwt_secret = std::env::var("JWT_ACCESS_SECRET")
            .unwrap_or_else(|_| "default_secret".to_string());

        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?;

        if token_data.claims.token_type != "refresh" {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken
            ));
        }

        Ok(token_data.claims)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    pub sub: String,        // User ID
    pub email: String,      // User email
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
    pub jti: String,        // JWT ID (unique identifier)
    pub token_type: String, // "access" or "refresh"
} 