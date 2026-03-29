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

    async fn c(&self) -> MultiplexedConnection {
        self.client
            .get_multiplexed_tokio_connection()
            .await
            .unwrap()
    }

    pub async fn incr_with_expire(&self, key: &str, window: usize) -> i32 {
        let mut c = self.c().await;
        let count: i32 = c.incr(key, 1).await.unwrap();
        if count == 1 {
            let _: () = c.expire(key, window as i64).await.unwrap();
        }
        count
    }

    pub async fn is_blocked(&self, key: &str) -> bool {
        self.c().await.exists(key).await.unwrap()
    }

    pub async fn block_ip(&self, key: &str, seconds: usize) {
        let _: () = self.c().await.set_ex(key, 1, seconds as u64).await.unwrap();
    }
}
