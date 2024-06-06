use crate::domain::{
    entity::parking_lot::ParkingLot, repository::parking_lot_repository::ParkingLotRepository,
};

pub struct EnterParkingLot {
    pub parking_log_respository: Box<dyn ParkingLotRepository>,
}

impl EnterParkingLot {
    pub async fn new(parking_log_respository: Box<dyn ParkingLotRepository>) -> Self {
        Self {
            parking_log_respository,
        }
    }

    pub async fn execute(&self, code: String) -> ParkingLot {
        self.parking_log_respository.get_parking_lot(code).await
    }
}
