// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_RATE_SERVICE_GET_RATES: ::grpcio::Method<super::pair::RateInput, super::pair::RatesOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/movie.RateService/GetRates",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RateServiceClient {
    client: ::grpcio::Client,
}

impl RateServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RateServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_rates_opt(&self, req: &super::pair::RateInput, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pair::RatesOutput> {
        self.client.unary_call(&METHOD_RATE_SERVICE_GET_RATES, req, opt)
    }

    pub fn get_rates(&self, req: &super::pair::RateInput) -> ::grpcio::Result<super::pair::RatesOutput> {
        self.get_rates_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_rates_async_opt(&self, req: &super::pair::RateInput, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pair::RatesOutput>> {
        self.client.unary_call_async(&METHOD_RATE_SERVICE_GET_RATES, req, opt)
    }

    pub fn get_rates_async(&self, req: &super::pair::RateInput) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pair::RatesOutput>> {
        self.get_rates_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RateService {
    fn get_rates(&mut self, ctx: ::grpcio::RpcContext, req: super::pair::RateInput, sink: ::grpcio::UnarySink<super::pair::RatesOutput>);
}

pub fn create_rate_service<S: RateService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_RATE_SERVICE_GET_RATES, move |ctx, req, resp| {
        instance.get_rates(ctx, req, resp)
    });
    builder.build()
}
