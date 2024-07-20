use log::debug;

use super::Plane;
use crate::decoder;

impl Plane {
    pub(super) fn update_from_mode_s(&mut self, message: &[u32], df: u32, relaxed: bool) {
        let mut bds = decoder::bds(message);
        if bds == (2, 0) {
            self.ais = decoder::ais(message);
        }
        if bds == (3, 0) {
            self.threat_encounter = decoder::threat_encounter(message);
        }
        if bds == (0, 0) {
            if let Some(result) = decoder::is_bds_1_7(message) {
                self.capability.1 = result;
                bds = (1, 7);
                debug!("Relaxed:{}", relaxed);
                debug!(
                    "DF:{}, BDS:{}.{}, C:{:b} 4:{} 4.4:{} 5:{} 6:{}",
                    df,
                    bds.0,
                    bds.1,
                    self.capability.1.flags,
                    self.capability.1.bds40,
                    self.capability.1.bds44,
                    self.capability.1.bds50,
                    self.capability.1.bds60
                );
            }
        }
        if bds == (0, 0) && (relaxed || self.capability.1.bds40) {
            if let Some(value) = decoder::is_bds_4_0(message) {
                self.selected_altitude =
                    value.mcp_selected_altitude.or(value.fms_selected_altitude);
                self.target_altitude_source = match value.target_altitude_source {
                    Some(v) => match v {
                        1 => '\u{2081}',
                        2 => '\u{2082}',
                        3 => '\u{2083}',
                        _ => ' ',
                    },
                    _ => ' ',
                };
                self.barometric_pressure_setting = value.barometric_pressure_setting;
                bds = (4, 0);
                debug!(
                    "DF:{}, BDS:{}.{} S:{}",
                    df,
                    bds.0,
                    bds.1,
                    value.target_altitude_source.unwrap_or(0)
                );
            }
        }
        if bds == (0, 0) && (relaxed || self.capability.1.bds50) {
            if let Some(result) = decoder::is_bds_5_0(message) {
                self.roll_angle = result.roll_angle;
                self.track = result.track_angle;
                self.track_angle_rate = result.track_angle_rate;
                self.grspeed = result.ground_speed;
                self.true_airspeed = result.true_airspeed;
                self.bds_5_0_timestamp = Some(self.timestamp);
                self.track_source = '\u{2085}';
                self.track_timestamp = Some(self.timestamp);
                bds = (5, 0);
                debug!("DF:{}, BDS:{}.{}", df, bds.0, bds.1);
            }
        }
        if bds == (0, 0) && (relaxed || self.capability.1.bds60) {
            if let Some(result) = decoder::is_bds_6_0(message) {
                self.heading = result.magnetic_heading;
                self.indicated_airspeed = result.indicated_airspeed;
                self.mach_number = result.mach_number;
                self.vrate = match result.barometric_altitude_rate.is_some() {
                    true => {
                        self.vrate_source = '\u{2086}';
                        result.barometric_altitude_rate
                    }
                    _ => {
                        self.vrate_source = '\u{2071}';
                        result.internal_vertical_velocity
                    }
                };
                self.heading_source = '\u{2086}';
                self.heading_timestamp = Some(self.timestamp);
                bds = (6, 0);
                debug!("DF:{}, BDS:{}.{}", df, bds.0, bds.1);
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
                debug!("DF:{} B:4.4 FOM:{:b}", df, message[8] & 0xF);
            }
        }
        if bds == (0, 0) {
            if let Some(value) = decoder::is_bds_4_5(message) {
                self.temperature = Some(value)
            }
        }
        debug!("DF:{} BDS:{}.{}", df, bds.0, bds.1);
    }
}
