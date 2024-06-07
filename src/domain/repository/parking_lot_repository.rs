use async_trait::async_trait;
use chrono::NaiveDateTime;

use crate::domain::entity::parking_lot::ParkingLot;

#[async_trait]
pub trait ParkingLotRepository: Send + Sync {
    async fn get_parking_lot(&self, code: String) -> ParkingLot;
    async fn save_parked_car(&mut self, code: String, plate: String, date: NaiveDateTime);
}
