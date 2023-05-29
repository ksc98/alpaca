// use crate::redis as _redis;
use log::{debug, error, info, warn};

#[tonic::async_trait]
pub trait EngineCache: Send + Sync {
    // fn get(&self, key: &str) -> Option<String>;
    // fn set(&self, key: &str, value: &str);
    // fn delete(&self, key: &str);
    fn scheduled_volume_buy(&self, ticker: &str, quantity: i32, timestamp: i64);
    fn scheduled_value_buy(&self, ticker: &str, quantity: i32, timestamp: i64);
}
