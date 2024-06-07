use async_trait::async_trait;
use chrono::NaiveDateTime;
use tokio::sync::RwLock;

use crate::{
    adapter::parking_lot_adapter::ParkingLotAdapter,
    domain::{
        entity::{parked_car::ParkedCar, parking_lot::ParkingLot},
        repository::parking_lot_repository::ParkingLotRepository,
    },
};

#[derive(Default)]
pub struct ParkingLotRepositoryInMemory {
    parking_lot: Vec<ParkingLot>,
    parked_cars: RwLock<Vec<ParkedCar>>,
}

impl ParkingLotRepositoryInMemory {
    pub fn new() -> Self {
        Self {
            parking_lot: vec![ParkingLotAdapter::create(
                "shopping_mall".to_string(),
                5,
                8,
                22,
                0,
            )],
            parked_cars: RwLock::new(vec![]),
        }
    }
}

#[async_trait]
impl ParkingLotRepository for ParkingLotRepositoryInMemory {
    async fn get_parking_lot(&self, code: String) -> ParkingLot {
        let parking_lot_data = self
            .parking_lot
            .iter()
            .find(|parking_lot| parking_lot.code == code)
            .unwrap();

        let occupied_spaces = self.parked_cars.read().await.len();

        let parking_lot = ParkingLotAdapter::create(
            parking_lot_data.code.clone(),
            parking_lot_data.capacity,
            parking_lot_data.open_hour,
            parking_lot_data.close_hour,
            occupied_spaces.try_into().unwrap(),
        );

        parking_lot.clone()
    }

    async fn save_parked_car(&mut self, code: String, plate: String, date: NaiveDateTime) {
        let mut parked_cars = self.parked_cars.write().await;
        parked_cars.push(ParkedCar::new(&code, plate, date).unwrap());

        if let Some(parking_lot) = self.parking_lot.iter_mut().find(|lot| lot.code == code) {
            parking_lot.occupied_spaces += 1;
        }
    }
}
