use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::{
    entity::parking_lot::ParkingLot, repository::parking_lot_repository::ParkingLotRepository,
};

pub struct GetParkingLot {
    pub parking_log_repository: Arc<RwLock<dyn ParkingLotRepository>>,
}

impl GetParkingLot {
    pub fn new(parking_log_repository: Arc<RwLock<dyn ParkingLotRepository>>) -> Self {
        Self {
            parking_log_repository,
        }
    }

    pub async fn execute(&self, code: String) -> ParkingLot {
        let repository = self.parking_log_repository.read().await;
        repository.get_parking_lot(code).await
    }
}
