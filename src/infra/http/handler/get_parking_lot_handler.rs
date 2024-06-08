use std::sync::Arc;

use axum::async_trait;
use tokio::sync::RwLock;

use crate::domain::{
    repository::parking_lot_repository::ParkingLotRepository,
    usecase::get_parking_lot::GetParkingLot,
};

use super::generic_handler::{AdapterRequest, AdapterResponse, GenericHandler};

pub struct GetParkingLotHandler {
    pub repository: Arc<RwLock<dyn ParkingLotRepository>>,
}

#[async_trait]
impl GenericHandler for GetParkingLotHandler {
    async fn handle(&self, request: AdapterRequest) -> AdapterResponse {
        let get_parking_lot = GetParkingLot::new(self.repository.clone());
        let parking_lot = get_parking_lot.execute(request.code).await;
        AdapterResponse { data: parking_lot }
    }
}
