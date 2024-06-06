use async_trait::async_trait;

use crate::domain::{
    entity::parking_lot::ParkingLot, repository::parking_lot_repository::ParkingLotRepository,
};

pub struct ParkingLotRepositoryInMemory;

#[async_trait]
impl ParkingLotRepository for ParkingLotRepositoryInMemory {
    async fn get_parking_lot(&self, code: String) -> ParkingLot {
        ParkingLot::new(code.as_str(), 5, 8, 22)
    }
}
