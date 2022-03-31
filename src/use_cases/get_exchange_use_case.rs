use crate::domain::pair::{Pairs, PairsFromAPI};
use crate::grpc_proto::pair::Pair;
use crate::use_cases::use_case::UseCase;
use curl::{easy::Easy, Error};

#[derive(Clone)]
pub struct GetExchangeUseCase {}

impl GetExchangeUseCase {
  pub fn new() -> Self {
    Self {}
  }
}

impl UseCase<&Vec<&str>, Result<Pairs, Error>> for GetExchangeUseCase {
  fn execute(&self, payload: &Vec<&str>) -> Result<Pairs, Error> {
    let mut buf = Vec::new();

    let mut easy = Easy::new();
    easy.url("https://api.binance.com/api/v3/ticker/price").unwrap();

    {
      let mut transfer = easy.transfer();
      transfer
        .write_function(|data| {
          buf.extend_from_slice(data);
          Ok(data.len())
        })
        .unwrap();
      transfer.perform().unwrap();
    }

    let res: PairsFromAPI = serde_json::from_slice(&buf).unwrap();

    let found_pairs: Vec<Pair> = res
      .iter()
      .filter(|&pair| {
        let pair = payload.into_iter().find(|&&v| v == &pair.symbol);
        match pair {
          Some(_) => true,
          None => false,
        }
      })
      .map(|pair| Pair {
        key: String::from(pair.symbol.as_str()),
        value: String::from(pair.price.as_str()),
        unknown_fields: Default::default(),
        cached_size: Default::default(),
      })
      .collect();
    Ok(found_pairs)
  }
}
