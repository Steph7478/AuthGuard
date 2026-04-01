use redis::{aio::MultiplexedConnection, AsyncCommands};

#[derive(Clone)]
pub struct RedisService {
    client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        Self {
            client: redis::Client::open(url).expect("Invalid Redis URL"),
        }
    }

    async fn conn(&self) -> MultiplexedConnection {
        self.client
            .get_multiplexed_tokio_connection()
            .await
            .expect("Failed to connect to Redis")
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.conn().await.get(key).await.ok()
    }

    pub async fn set(&self, key: &str, value: &str, ttl: u64) -> Result<(), redis::RedisError> {
        self.conn().await.set_ex(key, value, ttl).await
    }
}
