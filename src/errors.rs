use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Erro de configuração: {0}")]
    Config(String),
    
    #[error("Erro de banco de dados: {0}")]
    Database(String),
    
    #[error("Erro de Redis: {0}")]
    Redis(String),
    
    #[error("Erro de validação: {0}")]
    Validation(String),
    
    #[error("Erro interno: {0}")]
    Internal(String),
    
    #[error("Erro de autenticação: {0}")]
    Auth(String),
    
    #[error("Recurso não encontrado: {0}")]
    NotFound(String),
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Validation(err.to_string())
    }
}
