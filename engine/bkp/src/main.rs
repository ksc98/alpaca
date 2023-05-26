#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate log;

mod redis;

use log::Level;

fn main() {
    env_logger::init();
    let conn = redis::connect();
    match conn {
        Ok(c) => {
            info!("Connected to redis")
        }
        Err(e) => {
            error!("{}", e)
        }
    }
    println!("Hello, world!");
}
