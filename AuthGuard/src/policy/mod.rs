use crate::services::RedisService;
use std::sync::Arc;

const RATE_LIMIT: u32 = 100;
const WINDOW_SECS: u64 = 60;

pub struct Policy {
    redis: Arc<RedisService>,
}

impl Policy {
    pub fn new(redis: Arc<RedisService>) -> Self {
        Self { redis }
    }
    
    pub async fn check_rate_limit(&self, ip: &str) -> bool {
        let key = format!("rate:{}", ip);
        
        let count = self.redis.get(&key).await
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        
        if count >= RATE_LIMIT as i64 {
            return false;
        }
        
        let _ = self.redis.incr(&key).await;
        
        if count == 0 {
            let _ = self.redis.expire(&key, WINDOW_SECS).await;
        }
        
        true
    }
}