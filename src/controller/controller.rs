use crate::services::manager::Manager;
use crate::grpc_proto::rate_grpc::RateService;
use crate::grpc_proto::rate::{RateReq, RatesRes, Pair};

use grpcio::{RpcContext, UnarySink};

#[derive(Clone)]
pub struct Controller {
    services: Manager
}

impl Controller {
    pub fn new(manager: Manager) -> Self {
        Self {
            services: manager,
        }
    }
}

impl RateService for Controller {
    fn get_rates(&mut self, ctx: RpcContext<'a>, req: RateReq, sink: UnarySink<RatesRes>) {
        let mut r = RatesRes::new();

        let pair = Pair{
            key: "USD".to_string(),
            value: "10".to_string(),
            unknown_fields: Default::default(),
            cached_size: Default::default()
        };

        let pairs = protobuf::RepeatedField::from_vec(vec![pair]);

        r.set_pairs(pairs);

        sink.success(r);
    }
}
