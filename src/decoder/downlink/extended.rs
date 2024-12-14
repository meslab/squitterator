mod display;
mod update;

#[derive(Debug)]
pub struct Ext {
    pub df: Option<u32>,
    pub icao: Option<u32>,
    pub capability: u32,
    pub message_type: (u32, u32),
    pub ais: Option<String>,
    pub category: Option<(u32, u32)>,
    pub cpr: Option<(u32, u32, u32)>,
    pub ground_movement: Option<f64>,
    pub grspeed: Option<u32>,
    pub track: Option<u32>,
    pub track_source: Option<char>,
    pub heading: Option<u32>,
    pub heading_source: Option<char>,
    pub altitude: Option<u32>,
    pub altitude_source: Option<char>,
    pub altitude_delta: Option<i32>,
    pub altitude_gnss: Option<u32>,
    pub vrate: Option<i32>,
    pub vrate_source: Option<char>,
    pub surveillance_status: Option<char>,
    pub adsb_version: Option<u32>,
}

impl Default for Ext {
    fn default() -> Self {
        Self::new()
    }
}

impl Ext {
    pub fn new() -> Self {
        Ext {
            df: None,
            icao: None,
            capability: 0,
            message_type: (0, 0),
            ais: None,
            category: None,
            cpr: None,
            ground_movement: None,
            grspeed: None,
            track: None,
            track_source: None,
            heading: None,
            heading_source: None,
            altitude: None,
            altitude_source: None,
            altitude_delta: None,
            altitude_gnss: None,
            vrate: None,
            vrate_source: None,
            surveillance_status: None,
            adsb_version: None,
        }
    }
}
