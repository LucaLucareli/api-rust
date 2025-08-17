use sea_orm::{DatabaseConnection, Database as SeaDatabase};
use crate::config::Config;
use crate::errors::Result;

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self> {
        let connection = SeaDatabase::connect(&config.database_url).await?;
        
        Ok(Self { connection })
    }
    
    pub async fn test_connection(&self) -> Result<()> {
        self.connection.ping().await?;
        Ok(())
    }
}
