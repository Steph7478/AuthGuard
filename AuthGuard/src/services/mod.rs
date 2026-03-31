use redis::{aio::MultiplexedConnection, AsyncCommands};

#[derive(Clone)]
pub struct RedisService {
    client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        Self {
            client: redis::Client::open(url).unwrap(),
        }
    }

    async fn conn(&self) -> MultiplexedConnection {
        self.client
            .get_multiplexed_tokio_connection()
            .await
            .unwrap()
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.conn().await.get(key).await.ok()
    }

    pub async fn set(&self, key: &str, value: &str, ttl_secs: u64) {
        let _: () = self
            .conn()
            .await
            .set_ex(key, value, ttl_secs)
            .await
            .unwrap();
    }

    pub async fn incr(&self, key: &str) -> Result<i64, redis::RedisError> {
        let mut conn = self.conn().await;
        conn.incr(key, 1).await
    }

    pub async fn expire(&self, key: &str, seconds: u64) -> Result<bool, redis::RedisError> {
        let mut conn = self.conn().await;
        conn.expire(key, seconds as i64).await
    }
}
