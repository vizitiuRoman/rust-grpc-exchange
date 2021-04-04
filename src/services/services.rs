use crate::domain::pair::Pairs;

use curl::Error;

pub trait ExchangeService {
    fn get_exchange_by_pairs(&self, pairs: &Vec<&str>) -> Result<Pairs, Error>;
    fn contain_pair(&self, symbol: &str, pairs: &Vec<&str>) -> bool;
}
