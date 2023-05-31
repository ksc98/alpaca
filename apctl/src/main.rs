use chrono::Utc;
use chrono_english::{parse_date_string, Dialect};

use clap::Parser;

use engine::{engine_client::EngineClient, BuyRequest};

#[macro_use]
extern crate log;
extern crate date_time_parser;

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
    #[arg(long, default_value_t = String::from("now"))]
    time: String,

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
    let ts = parse_time(&args.time);
    let qv = match args.quantity > 0 {
        true => Some(crate::engine::buy_request::Qv::Quantity(args.quantity)),
        false => Some(crate::engine::buy_request::Qv::Value(args.value)),
    };
    let request = tonic::Request::new(BuyRequest {
        ticker: args.ticker.to_string(),
        qv,
        exchange: args.exchange,
        time: ts,
    });

    // let response: Response<BuyResponse> = Response;
    if ts <= chrono::offset::Utc::now().timestamp() {
        let response = client.buy(request).await?;
        println!("Response from server: \n{}", response.into_inner().message);
    } else {
        let response = client.schedule(request).await?;
        println!("Response from server: \n{}", response.into_inner().message);
    }
    Ok(())
}

fn parse_time(expression: &str) -> i64 {
    match parse_date_string(expression, Utc::now(), Dialect::Us) {
        Ok(time) => time.timestamp(),
        Err(e) => {
            println!("Error parsing time: {}", e);
            0
        }
    }
}
