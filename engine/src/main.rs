#[warn(dead_code)]
pub mod engine {
    tonic::include_proto!("engine");
}
mod cache;
mod engine_service;
mod redis_cache;

use engine::engine_server::EngineServer;
use engine_service::EngineService;
use env_logger::Builder as Logger;
use log::{debug, error, info, warn};
use redis_cache::RedisCache;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut logger = Logger::new();
    logger.filter_level(log::LevelFilter::Info);
    logger.init();

    let address = "0.0.0.0:4321".parse().unwrap();
    let backend = RedisCache::new();
    let engine_service = EngineService::new(backend);

    info!("starting server...");
    Server::builder()
        .add_service(EngineServer::new(engine_service))
        .serve(address)
        .await?;
    Ok(())
}
