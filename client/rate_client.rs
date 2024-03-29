extern crate grpcio;

#[path = "../src/grpc_proto/pair_grpc.rs"]
mod pair_grpc;

#[path = "../src/grpc_proto/pair.rs"]
mod pair;

use crate::pair_grpc::RateServiceClient;

use crate::pair::RateInput;
use dotenv::dotenv;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::{str::FromStr, sync::Arc};

fn main() {
  dotenv().ok();

  let env = Arc::new(EnvBuilder::new().build());
  let port_str = std::env::var("PORT").unwrap();
  let port = u16::from_str(&port_str).unwrap();

  let ch =
    ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());
  let client = RateServiceClient::new(ch);

  let mut input = RateInput::new();

  input.set_pairs(String::from("BTC-USDT,ETH-USDT"));

  let check = client.get_rates(&input).expect("RPC Failed!");
  println!("Ate {:?} and got charged ${:?}", input, check.get_pairs());
}
