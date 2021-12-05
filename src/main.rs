mod delivery;
mod domain;
mod grpc_proto;
mod services;

use delivery::grpc::grpc::GRPC;
use dotenv::dotenv;
use grpcio::{Environment, ServerBuilder};
use log::{info, LevelFilter};
use services::{exchange_service, manager};
use std::{str::FromStr, sync::Arc};

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    Config,
};

fn main() {
    dotenv().ok();

    let env = Arc::new(Environment::new(1));
    let port_str = std::env::var("PORT").unwrap();
    let port = u16::from_str(&port_str).unwrap();

    let logfile = FileAppender::builder().build("logs/output.log").unwrap();

    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("stdout").appender("logfile").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(config).unwrap();

    let addr: String = format!("0.0.0.0:{}", port).parse().expect("Invalid address");
    info!("Listening on http://{}", addr);

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
