use crate::{
    grpc_proto::{
        pair::{RateReq, RatesRes},
        pair_grpc::RateService,
    },
    services::manager::Manager,
};

use crate::services::services::ExchangeService;
use grpcio::{RpcContext, UnarySink};
use protobuf::RepeatedField;

#[derive(Clone)]
pub struct GRPC {
    services: Manager,
}

impl GRPC {
    pub fn new(manager: Manager) -> Self {
        Self { services: manager }
    }
}

impl RateService for GRPC {
    fn get_rates(&mut self, ctx: RpcContext<'a>, req: RateReq, sink: UnarySink<RatesRes>) {
        let mut r = RatesRes::new();

        let prepared_str = String::from(req.pairs).replace("-", "");
        let splitted: Vec<&str> = prepared_str.split(",").collect();
        let pairs = self.services.exchange_service.get_exchange_by_pairs(&splitted).unwrap();

        r.set_pairs(RepeatedField::from(pairs));

        sink.success(r);
    }
}
