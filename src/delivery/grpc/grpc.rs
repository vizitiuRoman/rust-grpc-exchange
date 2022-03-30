use crate::grpc_proto::{
  pair::{RateReq, RatesRes},
  pair_grpc::RateService,
};

use crate::use_cases::get_exchange_use_case::GetExchangeUseCase;
use crate::use_cases::use_case::UseCase;
use grpcio::{RpcContext, UnarySink};
use protobuf::RepeatedField;

#[derive(Clone)]
pub struct GRPC {
  get_exchange_use_case: GetExchangeUseCase,
}

impl GRPC {
  pub fn new(get_exchange_use_case: GetExchangeUseCase) -> Self {
    Self { get_exchange_use_case }
  }
}

impl RateService for GRPC {
  fn get_rates(
    &mut self,
    _: RpcContext,
    req: RateReq,
    sink: UnarySink<RatesRes>,
  ) {
    let mut r = RatesRes::new();

    let prepared_str = String::from(req.pairs).replace("-", "");
    let splitted: Vec<&str> = prepared_str.split(",").collect();
    let pairs = self.get_exchange_use_case.execute(&splitted).unwrap();

    r.set_pairs(RepeatedField::from(pairs));

    sink.success(r);
  }
}
