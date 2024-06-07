use chrono::NaiveDateTime;
use regex::Regex;

pub struct ParkedCar {
    pub code: String,
    pub plate: String,
    pub date: NaiveDateTime,
}

impl ParkedCar {
    pub fn new(code: &str, plate: String, date: NaiveDateTime) -> Result<Self, String> {
        let plate_regex = Regex::new(r"^[A-Z]{3}[0-9]{4}$").unwrap();
        if !plate_regex.is_match(plate.as_str()) {
            return Err("Invalid plate".into());
        }

        Ok(ParkedCar {
            code: code.into(),
            plate,
            date,
        })
    }
}
