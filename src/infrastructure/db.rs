use crate::config::SqlConfig;
use sea_orm::{Database, DatabaseConnection};
use std::error::Error;

pub async fn connect_sql(config: &SqlConfig) -> Result<DatabaseConnection, Box<dyn Error>> {
    let database_url = format!(
        "sqlserver://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.database
    );

    let db = Database::connect(&database_url).await?;

    Ok(db)
}
