use crate::config::Config;
use crate::errors::Result;

pub struct RedisClient {
    pub url: String,
}

impl RedisClient {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(Self { 
            url: config.redis_url.clone() 
        })
    }
    
    pub async fn test_connection(&self) -> Result<()> {
        // Implementação básica para teste
        println!("Redis URL: {}", self.url);
        Ok(())
    }
}
