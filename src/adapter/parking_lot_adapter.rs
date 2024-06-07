use crate::domain::entity::parking_lot::ParkingLot;

pub struct ParkingLotAdapter;

impl ParkingLotAdapter {
    pub fn create(
        code: String,
        capacity: i32,
        open_hours: i32,
        close_hours: i32,
        occupied_spaces: i32,
    ) -> ParkingLot {
        ParkingLot::new(
            code.as_str(),
            capacity,
            open_hours,
            close_hours,
            occupied_spaces,
        )
    }
}
