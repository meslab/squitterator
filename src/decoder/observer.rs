use lazy_static::lazy_static;
use log::error;
use std::str::FromStr;
use std::sync::Mutex;

pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

impl FromStr for Coordinates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Coordinates should be in the format lat,lon".to_string());
        }
        let lat = parts[0]
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<f64>()
            .map_err(|_| "Invalid latitude".to_string())?;
        let lon = parts[1]
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<f64>()
            .map_err(|_| "Invalid longitude".to_string())?;
        Ok(Coordinates { lat, lon })
    }
}

// Define the global variables for observer's latitude and longitude
lazy_static! {
    static ref OBSERVER_COORDS: Mutex<Option<(f64, f64)>> = Mutex::new(None);
}

// Function to set the observer's coordinates
fn set_observer_coords(c: Option<(f64, f64)>) {
    let mut coords = OBSERVER_COORDS
        .lock()
        .expect("Cannot set observer's coordinates.");
    *coords = c;
}

// Function to set the observer's coordinates from an argument string
pub fn set_observer_coords_from_str(coord_str: &str) {
    if let Err(e) = coord_str
        .parse::<Coordinates>()
        .map(|coords| set_observer_coords(Some((coords.lat, coords.lon))))
    {
        error!("Error parsing coordinates: {}", e);
    }
}

// Function to get the observer's coordinates
pub(crate) fn get_observer_coords() -> Option<(f64, f64)> {
    *OBSERVER_COORDS
        .lock()
        .expect("Failed to lock observer coordinates")
}
