#![feature(in_band_lifetimes)]

mod delivery;
mod domain;
mod grpc_proto;
mod services;

use delivery::grpc::grpc::GRPC;
use dotenv::dotenv;
use grpcio::{Environment, ServerBuilder};
use services::{exchange_service, manager};
use std::{str::FromStr, sync::Arc};

fn main() {
    dotenv().ok();

    let env = Arc::new(Environment::new(1));
    let port_str = std::env::var("PORT").unwrap();

    let port = u16::from_str(&port_str).unwrap();

    let controller = GRPC::new(manager::Manager::new(exchange_service::Exchange::new()));

    let service = grpc_proto::pair_grpc::create_rate_service(controller);

    let mut server =
        ServerBuilder::new(env).register_service(service).bind("0.0.0.0", port).build().unwrap();
    server.start();

    println!("Service started on port: {}", port,);

    loop {
        std::thread::park();
    }
}
