mod from_downlink;
mod from_squitter;
mod header;
mod legend;
mod simple_display;
mod update_position;

pub use from_downlink::UpdateFromDownlink;
pub use header::{DisplayFlags, LegendHeaders};
pub use legend::Legend;
pub use simple_display::format_simple_display;

use super::DF;
use crate::decoder::Capability;
use chrono::{DateTime, Utc};
use std::fmt::{self, Display};

pub struct Plane {
    pub icao: u32,
    pub capability: (u32, Capability),
    pub category: (u32, u32),
    pub reg: &'static str,
    pub ais: Option<String>,
    pub altitude: Option<u32>,
    pub altitude_gnss: Option<u32>,
    pub altitude_source: char,
    pub selected_altitude: Option<u32>,
    pub barometric_pressure_setting: Option<u32>,
    pub target_altitude_source: char,
    pub squawk: Option<u32>,
    pub surveillance_status: char,
    pub threat_encounter: Option<char>,
    pub vrate: Option<i32>,
    pub vrate_source: char,
    pub cpr_lat: [u32; 2],
    pub cpr_lon: [u32; 2],
    pub cpr_time: [DateTime<Utc>; 2],
    pub lat: f64,
    pub lon: f64,
    pub distance_from_observer: Option<f64>,
    pub grspeed: Option<u32>,
    pub true_airspeed: Option<u32>,
    pub indicated_airspeed: Option<u32>,
    pub mach_number: Option<f64>,
    pub ground_movement: Option<f64>,
    pub turn: u32,
    pub track: Option<u32>,
    pub track_source: char,
    pub heading: Option<u32>,
    pub heading_source: char,
    pub roll_angle: Option<i32>,
    pub track_angle_rate: Option<i32>,
    pub bds_5_0_timestamp: Option<DateTime<Utc>>,
    pub temperature: Option<f64>,
    pub wind: Option<(u32, u32)>,
    pub turbulence: Option<u32>,
    pub humidity: Option<u32>,
    pub pressure: Option<u32>,
    pub timestamp: DateTime<Utc>,
    pub position_timestamp: Option<DateTime<Utc>>,
    pub track_timestamp: Option<DateTime<Utc>>,
    pub heading_timestamp: Option<DateTime<Utc>>,
    pub last_type_code: u32,
    pub last_df: u32,
    pub adsb_version: Option<u32>,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            icao: 0,
            capability: (0, Capability::default()),
            category: (0, 0),
            reg: "",
            ais: None,
            altitude: None,
            altitude_gnss: None,
            altitude_source: ' ',
            selected_altitude: None,
            barometric_pressure_setting: None,
            target_altitude_source: ' ',
            squawk: None,
            surveillance_status: ' ',
            threat_encounter: None,
            vrate: None,
            vrate_source: '_',
            cpr_lat: [0, 0],
            cpr_lon: [0, 0],
            cpr_time: [Utc::now(), Utc::now()],
            lat: 0.0,
            lon: 0.0,
            distance_from_observer: None,
            grspeed: None,
            true_airspeed: None,
            indicated_airspeed: None,
            mach_number: None,
            ground_movement: None,
            turn: 0,
            track: None,
            track_source: ' ',
            heading: None,
            heading_source: ' ',
            roll_angle: None,
            track_angle_rate: None,
            bds_5_0_timestamp: None,
            temperature: None,
            wind: None,
            turbulence: None,
            humidity: None,
            pressure: None,
            timestamp: Utc::now(),
            position_timestamp: None,
            track_timestamp: None,
            heading_timestamp: None,
            last_type_code: 0,
            last_df: 0,
            adsb_version: None,
        }
    }

    pub fn from_get_message(message: &[u32], df: u32, icao: u32, relaxed: bool) -> Self {
        let mut plane = Plane::new();
        plane.icao = icao;
        (_, plane.reg) = super::icao_to_country(icao);
        plane.update(message, df, relaxed);
        plane
    }

    pub fn from_downlink(dl: &DF, icao: u32) -> Self {
        let mut plane = Plane::new();
        plane.icao = icao;
        (_, plane.reg) = super::icao_to_country(icao);
        plane.update_from_downlink(dl);
        plane
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ICAO: {:06X}", self.icao)?;
        write!(f, " Reg: {:2}", self.reg)?;
        if let Some(altitude) = self.altitude {
            write!(f, " Alt: {:>5}", altitude)?;
        } else {
            write!(f, " {:10}", "")?;
        }
        if let Some(squawk) = self.squawk {
            write!(f, " Squawk: {:04}", squawk)?;
        } else {
            write!(f, " {:12}", "")?;
        }
        if let Some(ais) = &self.ais {
            write!(f, " AIS: {:8}", ais)?;
        } else {
            write!(f, " {:13}", "")?;
        }
        if self.lat != 0.0 && self.lon != 0.0 {
            write!(f, " Lat: {:10.6} Lon: {:11.6}", self.lat, self.lon)?;
        } else {
            write!(f, " {:15} {:16}", "", "")?;
        }
        if let Some(grspeed) = self.grspeed {
            write!(f, " GSpd: {:>3.0}", grspeed)?;
        } else {
            write!(f, " {:19}", "")?;
        }
        if let Some(track) = self.track {
            write!(f, " Track: {:>3.0}", track)?;
        } else {
            write!(f, " {:15}", "")?;
        }
        write!(f, "")
    }
}
