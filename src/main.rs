#![feature(in_band_lifetimes)]

mod controller;
mod services;
mod grpc_proto;

use grpcio::{Environment, ServerBuilder};
use controller::controller::Controller;
use services::{manager, rate_service};
use std::sync::Arc;

fn main() {

    let env = Arc::new(Environment::new(1));

    let port = 7070;

    let controller = Controller::new(
        manager::Manager::new(
            rate_service::RateSrv::new()
        )
    );

    let service = grpc_proto::rate_grpc::create_rate_service(controller);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", port)
        .build()
        .unwrap();
    server.start();

    println!(
        "Service started on port: {}",
        port,
    );

    loop {
        std::thread::park();
    }
}
