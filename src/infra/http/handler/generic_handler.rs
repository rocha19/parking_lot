use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::entity::parking_lot::ParkingLot;

#[derive(Deserialize)]
pub struct AdapterRequest {
    pub code: String,
}

#[derive(Serialize)]
pub struct AdapterResponse {
    pub data: ParkingLot,
}

#[async_trait]
pub trait GenericHandler {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse;
}
