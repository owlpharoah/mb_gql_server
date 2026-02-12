use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Cache {
    client: ConnectionManager,
}

impl Cache {
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let manager = ConnectionManager::new(client).await?;
        Ok(Self { client: manager })
    }

    // get
    pub async fn get<T: for<'de> Deserialize<'de>>(&mut self, key: &str) -> Option<T> {
        let result: Option<String> = self.client.get(key).await.ok()?;
        result.and_then(|json| serde_json::from_str(&json).ok())
    }

    // set
    pub async fn set<T: Serialize>(&mut self, key: &str, value: &T, ttl_seconds: u64) -> bool {
        let json = match serde_json::to_string(value) {
            Ok(j) => j,
            Err(_) => return false,
        };
        
         let result: Result<(), redis::RedisError> = self.client
            .set_ex(key, json, ttl_seconds)
            .await;
        
        result.is_ok()
    }

    pub fn artist_key(gid: &str) -> String {
        format!("mb:artist:{}", gid)
    }

    pub fn release_key(gid: &str) -> String {
        format!("mb:release:{}", gid)
    }
}