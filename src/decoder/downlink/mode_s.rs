use log::debug;

use crate::decoder::{self, Capability};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Mds {
    pub df: Option<u32>,
    pub icao: Option<u32>,
    pub altitude: Option<u32>,
    pub ais: Option<String>,
    pub threat_encounter: Option<char>,
    pub capability: Option<Capability>,
    pub selected_altitude: Option<u32>,
    pub target_altitude_source: Option<char>,
    pub barometric_pressure_setting: Option<u32>,
    pub roll_angle: Option<i32>,
    pub track: Option<u32>,
    pub track_angle_rate: Option<i32>,
    pub grspeed: Option<u32>,
    pub true_airspeed: Option<u32>,
    pub track_source: Option<char>,
    pub heading: Option<u32>,
    pub indicated_airspeed: Option<u32>,
    pub mach_number: Option<f64>,
    pub vrate: Option<i32>,
    pub vrate_source: Option<char>,
    pub heading_source: Option<char>,
    pub temperature: Option<f64>,
    pub wind: Option<(u32, u32)>,
    pub humidity: Option<u32>,
    pub turbulence: Option<u32>,
    pub pressure: Option<u32>,
}

impl Default for Mds {
    fn default() -> Self {
        Self::new()
    }
}

impl Mds {
    pub fn new() -> Self {
        Mds {
            df: None,
            icao: None,
            altitude: None,
            ais: None,
            threat_encounter: None,
            capability: None,
            selected_altitude: None,
            target_altitude_source: None,
            barometric_pressure_setting: None,
            roll_angle: None,
            track: None,
            track_angle_rate: None,
            grspeed: None,
            true_airspeed: None,
            track_source: None,
            heading: None,
            indicated_airspeed: None,
            mach_number: None,
            vrate: None,
            vrate_source: None,
            heading_source: None,
            temperature: None,
            wind: None,
            humidity: None,
            turbulence: None,
            pressure: None,
        }
    }
}

impl Display for Mds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(v) = self.df {
            write!(f, "DF{:02}", v)?
        } else {
            write!(f, "")?
        }
        if let Some(v) = self.icao {
            write!(f, ",{:X}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.altitude {
            writeln!(f, ",{}", v)
        } else {
            writeln!(f, ",")
        }
    }
}

impl decoder::Downlink for Mds {
    fn from_message(message: &[u32]) -> Result<Self, &str> {
        let mut dl = Mds::new();
        dl.update(message);
        Ok(dl)
    }

    fn update(&mut self, message: &[u32]) {
        if let Some(df) = decoder::df(message) {
            self.df = Some(df);
            self.icao = decoder::icao(message, df);
            self.altitude = decoder::altitude(message, df);
        }

        let mut bds = decoder::bds(message);
        if bds == (2, 0) {
            self.ais = decoder::ais(message);
        }
        if bds == (3, 0) {
            self.threat_encounter = decoder::threat_encounter(message);
        }
        if bds == (0, 0) {
            if let Some(result) = decoder::is_bds_1_7(message) {
                self.capability = Some(result);
                bds = (1, 7);
            }
        }
        if bds == (0, 0) {
            if let Some(value) = decoder::is_bds_4_0(message) {
                self.selected_altitude =
                    value.mcp_selected_altitude.or(value.fms_selected_altitude);
                self.target_altitude_source = match value.target_altitude_source {
                    Some(v) => match v {
                        1 => Some('\u{2081}'),
                        2 => Some('\u{2082}'),
                        3 => Some('\u{2083}'),
                        _ => Some(' '),
                    },
                    _ => Some(' '),
                };
                self.barometric_pressure_setting = value.barometric_pressure_setting;
                bds = (4, 0);
            }
        }
        if bds == (0, 0) {
            if let Some(result) = decoder::is_bds_5_0(message) {
                self.roll_angle = result.roll_angle;
                self.track = result.track_angle;
                self.track_angle_rate = result.track_angle_rate;
                self.grspeed = result.ground_speed;
                self.true_airspeed = result.true_airspeed;
                self.track_source = Some('\u{2085}');
                bds = (5, 0);
            }
        }
        if bds == (0, 0) {
            if let Some(result) = decoder::is_bds_6_0(message) {
                self.heading = result.magnetic_heading;
                self.indicated_airspeed = result.indicated_airspeed;
                self.mach_number = result.mach_number;
                self.vrate = match result.barometric_altitude_rate.is_some() {
                    true => {
                        self.vrate_source = Some('\u{2086}');
                        result.barometric_altitude_rate
                    }
                    _ => {
                        self.vrate_source = Some('\u{2071}');
                        result.internal_vertical_velocity
                    }
                };
                self.heading_source = Some('\u{2086}');
                bds = (6, 0);
            }
        }
        if bds == (0, 0) {
            if let Some(meteo) = decoder::is_bds_4_4(message) {
                self.temperature = meteo.temp;
                if meteo.wind.is_some() {
                    self.wind = meteo.wind;
                }
                self.humidity = meteo.humidity;
                self.turbulence = meteo.turbulence;
                self.pressure = meteo.pressure;
                bds = (4, 4);
            }
        }
        if bds == (4, 5) {
            self.temperature = decoder::is_bds_4_5(message);
        }
        debug!("DF:{} BDS:{}.{}", self.df.unwrap_or(0), bds.0, bds.1);
    }

    fn icao(&self) -> Option<u32> {
        self.icao
    }
}
