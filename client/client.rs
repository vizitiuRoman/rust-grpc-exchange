extern crate grpcio;

#[path = "../src/grpc_proto/pair_grpc.rs"]
mod pair_grpc;

#[path = "../src/grpc_proto/pair.rs"]
mod pair;

use crate::{
    pair_grpc::RateServiceClient,
};

use dotenv::dotenv;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::{str::FromStr, sync::Arc};
use crate::pair::RateReq;

fn main() {
    dotenv().ok();

    let env = Arc::new(EnvBuilder::new().build());
    let port_str = std::env::var("PORT").unwrap();
    let port = u16::from_str(&port_str).unwrap();

    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());
    let client = RateServiceClient::new(ch);

    let mut req = RateReq::new();

    req.set_pairs(String::from("BTC-USDT,ETH-USDT"));

    let check = client.get_rates(&req).expect("RPC Failed!");
    println!("Ate {:?} and got charged ${:?}", req, check.get_pairs());
}
