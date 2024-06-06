use async_trait::async_trait;

use crate::domain::entity::parking_lot::ParkingLot;

#[async_trait]
pub trait ParkingLotRepository {
    async fn get_parking_lot(&self, code: String) -> ParkingLot;
}
