use std::fmt;

#[derive(Debug, Clone)]
pub struct Position {
    pub time: String,
    pub lat: f64,
    pub lon: f64,
}

impl Position {
    pub fn new(time: String, lat: f64, lon: f64) -> Position {
        Position { time, lat, lon }
    }

    pub fn default() -> Position {
        Position::new(String::from(""), 0.0, 0.0)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.lat == other.lat && self.lon == other.lon
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
