use chrono::{NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ParkingLot {
    pub code: String,
    pub capacity: i32,
    pub open_hour: i32,
    pub close_hour: i32,
    pub occupied_spaces: i32,
}

impl ParkingLot {
    pub fn new(
        code: &str,
        capacity: i32,
        open_hour: i32,
        close_hour: i32,
        occupied_spaces: i32,
    ) -> Self {
        Self {
            code: code.to_string(),
            capacity,
            open_hour,
            close_hour,
            occupied_spaces,
        }
    }

    pub fn is_open(&self, date: NaiveDateTime) -> bool {
        let hour = date.hour() as i32;
        hour >= self.open_hour && hour < self.close_hour
    }

    pub fn is_full(&self) -> bool {
        self.occupied_spaces >= self.capacity
    }
}
