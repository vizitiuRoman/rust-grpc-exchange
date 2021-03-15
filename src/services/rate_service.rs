use crate::services::services::RateServiceTrait;

#[derive(Clone)]
pub struct RateSrv {
}

impl RateSrv {
    pub fn new() -> Self {
        Self {}
    }
}

impl RateServiceTrait for RateSrv {

}
