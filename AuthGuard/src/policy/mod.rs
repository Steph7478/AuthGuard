use crate::services::RedisService;
use std::sync::Arc;

pub struct Policy {
    redis: Arc<RedisService>,
    rate_limit_per_minute: u32,
}

impl Policy {
    pub fn new(redis: Arc<RedisService>) -> Self {
        Self {
            redis,
            rate_limit_per_minute: 100,
        }
    }

    pub async fn check_rate_limit(&self, ip: &str) -> bool {
        let key = format!("rate_limit:{}", ip);

        let count = self.redis.get(&key).await
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        if count >= self.rate_limit_per_minute as i64 {
            return false;
        }

        let _ = self.redis.incr(&key).await;

        if count == 0 {
            let _ = self.redis.expire(&key, 60).await;
        }

        true
    }
}

pub fn load_policy(redis: Arc<RedisService>) -> Policy {
    Policy::new(redis)
}