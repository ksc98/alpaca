extern crate redis;

use crate::cache::EngineCache;
use log::{debug, error, info, warn};
use redis::Commands;
use redis::Value as RV;
use redis::{Connection, RedisResult};
use std::env;

macro_rules! redis_url {
    ($name:expr, $default:expr) => {
        format!(
            "redis://{}/",
            env::var($name).unwrap_or_else(|_| String::from($default))
        )
    };
}

pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn new() -> Box<dyn EngineCache> {
        let redis_url = redis_url!("REDIS_HOST", "localhost");
        let client = redis::Client::open(redis_url).unwrap();
        let _ = client.get_connection().unwrap();
        info!("connected to redis!");
        Box::new(RedisCache { client })
    }
}

#[tonic::async_trait]
impl EngineCache for RedisCache {
    fn scheduled_volume_buy(&self, ticker: &str, quantity: i32, timestamp: i64) {
        let mut conn = self.client.get_connection().unwrap();
        let _: RedisResult<RV> = conn.hset(
            "buy_list_quantity",
            ticker,
            format!("{}:{}", timestamp, quantity),
        );

        let a: Vec<(String, String)> = conn.hgetall("buy_list_quantity").unwrap();
        println!("{:?}", a);
    }
    fn scheduled_value_buy(&self, ticker: &str, value: i32, timestamp: i64) {
        let mut conn = self.client.get_connection().unwrap();
        let _: RedisResult<RV> =
            conn.hset("buy_list_value", ticker, format!("{}:{}", timestamp, value));

        let a: Vec<(String, String)> = conn.hgetall("buy_list_value").unwrap();
        println!("{:?}", a);
    }
}
