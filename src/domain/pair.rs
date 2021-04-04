use crate::grpc_proto::pair::Pair;
use serde::{Deserialize, Serialize};

pub type Pairs = Vec<Pair>;

pub type PairsFromAPI = Vec<PairFromAPI>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PairFromAPI {
    pub symbol: String,
    pub price: String,
}
