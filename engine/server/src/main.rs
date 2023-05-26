use engine::{
    engine_server::{Engine, EngineServer},
    BuyRequest, BuyResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod engine {
    tonic::include_proto!("engine");
}

#[derive(Debug, Default)]
pub struct EngineService {}

#[tonic::async_trait]
impl Engine for EngineService {
    async fn buy(&self, request: Request<BuyRequest>) -> Result<Response<BuyResponse>, Status> {
        let r = request.into_inner();
        println!("received buy request for {}", r.ticker);
        Ok(Response::new(engine::BuyResponse {
            message: {
                format!(
                    "Ticker: {}\nValue: {}\nQuantity: {}\nTime: {}\nExchange: {}",
                    r.ticker, r.value, r.quantity, r.time, r.exchange,
                )
            },
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "0.0.0.0:4321".parse().unwrap();
    let engine_service = EngineService::default();

    println!("starting server...");
    Server::builder()
        .add_service(EngineServer::new(engine_service))
        .serve(address)
        .await?;
    Ok(())
}
