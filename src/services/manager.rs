use crate::services::rate_service::RateSrv;

#[derive(Clone)]
pub struct Manager {
    pub rate_service: RateSrv
}

impl Manager {
    pub fn new(rate_service: RateSrv) -> Self {
        Self {
            rate_service,
        }
    }
}
