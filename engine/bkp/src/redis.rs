extern crate redis;
use redis::{Connection, RedisResult};

pub fn connect() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    client.get_connection()
}
