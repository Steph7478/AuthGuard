use crate::config::AppConfig;
use crate::policy::Policy;
use crate::services::RedisService;
use std::sync::Arc;

pub struct AppState {
    pub config: Arc<AppConfig>,
    pub redis: Arc<RedisService>,
    pub policy: Arc<Policy>,
}
