use chrono::NaiveDateTime;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    entity::{parked_car::ParkedCar, parking_lot::ParkingLot},
    repository::parking_lot_repository::ParkingLotRepository,
};

pub struct EnterParkingLot {
    pub parking_log_repository: Arc<RwLock<dyn ParkingLotRepository>>,
}

impl EnterParkingLot {
    pub fn new(parking_log_repository: Arc<RwLock<dyn ParkingLotRepository>>) -> Self {
        Self {
            parking_log_repository,
        }
    }

    pub async fn execute(
        &self,
        code: String,
        plate: String,
        date: NaiveDateTime,
    ) -> Result<ParkingLot, String> {
        let mut repository = self.parking_log_repository.write().await;

        let parking_lot = repository.get_parking_lot(code.clone()).await;
        let parked_car = ParkedCar::new(&code, plate, date).unwrap();

        if !parking_lot.is_open(parked_car.date) {
            return Err("parking lot is not open".to_string());
        }

        if parking_lot.is_full() {
            return Err("parking lot is full".to_string());
        }

        repository
            .save_parked_car(parked_car.code, parked_car.plate, parked_car.date)
            .await;

        Ok(parking_lot)
    }
}
