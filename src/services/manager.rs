use crate::services::exchange_service::Exchange;

#[derive(Clone)]
pub struct Manager {
    pub exchange_service: Exchange,
}

impl Manager {
    pub fn new(exchange_service: Exchange) -> Self {
        Self { exchange_service }
    }
}
