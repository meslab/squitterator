use super::Plane;
use crate::decoder;
use std::f64::consts::PI;

/// Updates the position of the plane based on the received message type and CPR format.
///
/// # Arguments
///
/// * `message_type` - The type of the received message.
/// * `cpr_form` - The CPR format.
///
/// # Remarks
///
/// This method updates the position of the plane if the following conditions are met:
/// - The CPR latitude and longitude values are not equal to zero.
/// - The time difference between the CPR timestamps is less than 10 seconds.
///
/// If the conditions are met and the calculated latitude and longitude values are within the valid range,
/// the plane's latitude, longitude, and position timestamp are updated.
///
impl Plane {
    pub(super) fn update_position(&mut self, message_type: u32, cpr_form: u32) {
        if self.cpr_lat[0] != 0
            && self.cpr_lat[1] != 0
            && self.cpr_lon[0] != 0
            && self.cpr_lon[1] != 0
            && self.cpr_time[0]
                .signed_duration_since(self.cpr_time[1])
                .num_seconds()
                .abs()
                < 10
            && let Some((lat, lon)) = match message_type {
                5..=8 => decoder::cpr_location(&self.cpr_lat, &self.cpr_lon, cpr_form, 4),
                9..=18 => decoder::cpr_location(&self.cpr_lat, &self.cpr_lon, cpr_form, 1),
                _ => None,
            }
            && (-90.0..=90.0).contains(&lat)
            && (-180.0..=180.0).contains(&lon)
        {
            self.lat = lat;
            self.lon = lon;
            if let Some(observer) = decoder::observer::get_observer_coords() {
                self.distance_from_observer =
                    Some(haversine(self.lat, self.lon, observer.0, observer.1));
            };
            self.position_timestamp = Some(self.timestamp);
        }
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Haversine formula to calculate the distance between two points
pub(super) fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // Earth radius in kilometers

    let lat1 = degrees_to_radians(lat1);
    let lon1 = degrees_to_radians(lon1);
    let lat2 = degrees_to_radians(lat2);
    let lon2 = degrees_to_radians(lon2);

    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}
