use crate::cache::EngineCache;
use crate::engine::{buy_request::Qv, engine_server::Engine, BuyRequest, BuyResponse};
use tonic::{Request, Response, Status};

pub struct EngineService {
    cache: Box<dyn EngineCache>,
}

impl EngineService {
    // takes a backend which implements the EngineCache trait
    pub fn new(cache: Box<dyn EngineCache>) -> EngineService {
        EngineService { cache }
    }
}

#[tonic::async_trait]
impl Engine for EngineService {
    async fn buy(&self, request: Request<BuyRequest>) -> Result<Response<BuyResponse>, Status> {
        let r = request.into_inner();
        println!("received buy request for {}", r.ticker);

        // execute buy req
        // TODO: hits the alpaca api

        Ok(Response::new(BuyResponse {
            message: {
                format!(
                    "Ticker: {}\nQuantity/Value: {:?}\nTime: {}\nExchange: {}",
                    r.ticker,
                    r.qv.unwrap(),
                    // r.quantity,
                    r.time,
                    r.exchange,
                )
            },
        }))
    }

    async fn schedule(
        &self,
        request: Request<BuyRequest>,
    ) -> Result<Response<BuyResponse>, Status> {
        let r = request.into_inner();
        println!("received schedule request for {}", r.ticker);

        match r.qv {
            Some(Qv::Value(value)) => {
                self.cache.scheduled_value_buy(&r.ticker, value, r.time);
            }
            Some(Qv::Quantity(quantity)) => {
                self.cache.scheduled_volume_buy(&r.ticker, quantity, r.time);
            }
            None => {}
        };

        Ok(Response::new(BuyResponse {
            message: {
                format!(
                    "Ticker: {}\nQuantity/Value: {:?}\nTime: {}\nExchange: {}",
                    r.ticker,
                    r.qv.unwrap(),
                    r.time,
                    r.exchange,
                )
            },
        }))
    }
}
