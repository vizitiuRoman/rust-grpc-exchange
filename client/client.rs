extern crate grpcio;

#[path = "../src/grpc_proto/rate_grpc.rs"]
mod rate_grpc;

#[path = "../src/grpc_proto/rate.rs"]
mod rate;


use crate::rate_grpc::{RateServiceClient};
use crate::rate::{RateReq, RatesRes};

use grpcio::{ChannelBuilder, EnvBuilder};
use std::env;
use std::sync::Arc;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", 7070).as_str());
    let client = RateServiceClient::new(ch);

    let mut pairs = RateReq::new();

    pairs.set_pairs(String::from("CACAO"));

    let check = client.get_rates(&pairs).expect("RPC Failed!");
    println!("Ate {:?} and got charged ${:?}", pairs, check.get_pairs());
}
