use chrono;
use clap::Parser;
use engine::{engine_client::EngineClient, BuyRequest};
#[macro_use]
extern crate log;
use log::Level;

// Alpaca cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // The ticker to submit a market order for
    #[arg(short, long)]
    ticker: String,

    // The quantity to purchase
    #[arg(short, long, default_value_t = -1)]
    quantity: i32,

    // The $ value to purchase (fractional)
    #[arg(short, long, default_value_t = -1)]
    value: i32,

    // The time the order should be submitted at
    #[arg(long, default_value_t = chrono::offset::Utc::now().timestamp())]
    time: i64,

    // The exchange the order should be placed on
    #[arg(short, long, default_value_t = String::from("alpaca"))]
    exchange: String,
}

pub mod engine {
    tonic::include_proto!("engine");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    let mut client = EngineClient::connect("http://0.0.0.0:4321").await?;
    let request = tonic::Request::new(BuyRequest {
        ticker: args.ticker.to_string(),
        value: args.value,
        quantity: args.quantity,
        exchange: args.exchange,
        time: args.time,
    });

    let response = client.buy(request).await?;
    println!("Response from server: \n{}", response.into_inner().message);
    Ok(())
}
