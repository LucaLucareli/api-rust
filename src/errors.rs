use thiserror::Error;

pub type Result<T> = std::result::Result<T, MigrationError>;

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Erro MongoDB: {0}")]
    MongoDB(#[from] mongodb::error::Error),

    #[error("Erro SQL: {0}")]
    Sql(#[from] tiberius::error::Error),

    #[error("Erro genérico: {0}")]
    Generic(#[from] anyhow::Error),
}
