use crate::errors::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub sql: SqlConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub database: u8,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let redis_host = env::var("REDIS_CACHE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let redis_port = env::var("REDIS_CACHE_PORT")
            .unwrap_or_else(|_| "27002".to_string())
            .parse::<u16>()
            .unwrap_or(27002);
        let redis_db = env::var("REDIS_QUEUE_DATABASE")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<u8>()
            .unwrap_or(1);

        let redis_url = format!("redis://{}:{}/{}", redis_host, redis_port, redis_db);

        Ok(AppConfig {
            redis: RedisConfig {
                host: redis_host,
                port: redis_port,
                database: redis_db,
                url: redis_url,
            },

            sql: SqlConfig {
                database_url: env::var("DB_URL").expect("DB_URL deve ser definida"),
                host: env::var("DB_HOST").expect("DB_HOST deve ser definida"),
                port: env::var("DB_PORT")
                    .expect("DB_PORT deve ser definida")
                    .parse::<u16>()
                    .expect("DB_PORT deve ser um número"),
                user: env::var("DB_USER").expect("DB_USER deve ser definida"),
                password: env::var("DB_PASSWORD").expect("DB_PASSWORD deve ser definida"),
                database: env::var("DB_NAME").expect("DB_NAME deve ser definida"),
            },
        })
    }
}