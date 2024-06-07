use regex::Regex;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plate {
    value: String,
}

#[allow(dead_code)]
impl Plate {
    pub fn new(plate: &str) -> Result<Self, String> {
        let plate_regex = Regex::new(r"^[A-Z]{3}[0-9]{4}$").unwrap();
        if !plate_regex.is_match(plate) {
            return Err("Invalid plate".into());
        }
        Ok(Plate {
            value: plate.into(),
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
