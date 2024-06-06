use async_trait::async_trait;

pub struct ParkingLot {
    pub code: String,
    capacity: i32,
    open_hour: i8,
    close_hour: i8,
}

impl ParkingLot {
    pub fn new(code: &str, capacity: i32, open_hour: i8, close_hour: i8) -> Self {
        Self {
            code: code.to_string(),
            capacity: 0,
            open_hour: 0,
            close_hour: 0,
        }
    }
}

#[async_trait]
pub trait ParkingLotRepository {
    async fn get_parking_lot(&self, code: String) -> ParkingLot;
}

pub struct ParkingLotRepositoryInMemory;

#[async_trait]
impl ParkingLotRepository for ParkingLotRepositoryInMemory {
    async fn get_parking_lot(&self, code: String) -> ParkingLot {
        ParkingLot::new(code.as_str(), 5, 8, 22)
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
