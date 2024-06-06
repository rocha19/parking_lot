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
