use crate::{
    domain::pair::{Pairs, PairsFromAPI},
    grpc_proto::pair,
    services::services::ExchangeService,
};
use curl::{easy::Easy, Error};

#[derive(Clone)]
pub struct Exchange {}

impl Exchange {
    pub fn new() -> Self {
        Self {}
    }
}

impl ExchangeService for Exchange {
    fn get_exchange_by_pairs(&self, pairs: &Vec<&str>) -> Result<Pairs, Error> {
        let mut buf = Vec::new();
        let mut handle = Easy::new();
        handle.url("https://api.binance.com/api/v3/ticker/price").unwrap();

        {
            let mut transfer = handle.transfer();
            transfer
                .write_function(|data| {
                    buf.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        let res: PairsFromAPI = serde_json::from_slice(&buf).unwrap();

        let found_pairs: Vec<pair::Pair> = res
            .iter()
            .filter(|&pair| self.contain_pair(&pair.symbol, &pairs))
            .map(|pair| pair::Pair {
                key: String::from(pair.symbol.as_str()),
                value: String::from(pair.price.as_str()),
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })
            .collect();
        Ok(found_pairs)
    }

    fn contain_pair(&self, symbol: &str, pairs: &Vec<&str>) -> bool {
        let pair = pairs.into_iter().find(|&&v| v == symbol);
        match pair {
            Some(_) => true,
            None => false,
        }
    }
}
