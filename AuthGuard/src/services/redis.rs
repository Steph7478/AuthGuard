use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisService {
    client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).unwrap();
        Self { client }
    }

    async fn conn(&self) -> redis::aio::Connection {
        self.client.get_async_connection().await.unwrap()
    }

    pub async fn incr_with_expire(&self, key: &str, window: usize) -> i32 {
        let mut conn = self.conn().await;
        let count: i32 = conn.incr(key, 1).await.unwrap();
        if count == 1 {
            let _: () = conn.expire(key, window).await.unwrap();
        }
        count
    }

    pub async fn is_blocked(&self, key: &str) -> bool {
        let mut conn = self.conn().await;
        conn.exists(key).await.unwrap()
    }

    pub async fn block_ip(&self, key: &str, seconds: usize) {
        let mut conn = self.conn().await;
        let _: () = conn.set_ex(key, 1, seconds).await.unwrap();
    }
}
