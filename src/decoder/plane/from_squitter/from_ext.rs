use log::debug;

use super::Plane;
use crate::decoder;

impl Plane {
    pub(super) fn update_from_ext(&mut self, message: &[u32], df: u32) {
        let (message_type, message_subtype) = decoder::get_message_type(message);
        self.last_type_code = message_type;
        debug!("DF:{}, TC:{}, ST:{}", df, message_type, message_subtype);
        match message_type {
            1..=4 => {
                self.update_from_ext_1_4(message, message_type, message_subtype);
            }
            5..=8 => {
                self.update_from_ext_5_8(message, message_type);
            }
            9..=18 => {
                self.update_from_ext_9_18(message, message_type, df);
            }
            19 => {
                self.update_from_ext_19(message, message_subtype);
            }
            20..=22 => {
                self.update_from_ext_20_22(message);
            }
            31 => {
                self.update_from_ext_31(message);
            }
            _ => {}
        }
    }

    fn update_cpr(&mut self, message: &[u32], message_type: u32) {
        if let Some((cpr_form, cpr_lat, cpr_lon)) =
            decoder::cpr(message).filter(|(cpr_form, _, _)| (0..=1).contains(cpr_form))
        {
            self.cpr_lat[cpr_form as usize] = cpr_lat;
            self.cpr_lon[cpr_form as usize] = cpr_lon;
            self.cpr_time[cpr_form as usize] = self.timestamp;

            self.update_position(message_type, cpr_form);
        }
    }

    pub(super) fn update_from_ext_1_4(
        &mut self,
        message: &[u32],
        message_type: u32,
        message_subtype: u32,
    ) {
        self.ais = decoder::ais(message);
        self.category = (message_type, message_subtype);
    }

    pub(super) fn update_from_ext_5_8(&mut self, message: &[u32], message_type: u32) {
        self.ground_movement = decoder::ground_movement(message);
        self.altitude = None;
        self.altitude_source = '\u{2070}';
        self.track = decoder::ground_track(message);
        self.track_source = ' ';
        self.update_cpr(message, message_type);
    }

    pub(super) fn update_from_ext_9_18(&mut self, message: &[u32], message_type: u32, df: u32) {
        self.altitude = decoder::altitude(message, df);
        self.altitude_source = ' ';
        self.surveillance_status = decoder::surveillance_status(message);
        self.update_cpr(message, message_type);
    }

    pub(super) fn update_from_ext_19(&mut self, message: &[u32], message_subtype: u32) {
        self.vrate = decoder::vertical_rate(message);
        self.vrate_source = ' ';
        if let Some(altitude) = self.altitude {
            if let Some(altitude_delta) = decoder::altitude_delta(message) {
                self.altitude_gnss = Some((altitude as i32 + altitude_delta) as u32);
            }
        }
        match message_subtype {
            1 => {
                (self.track, self.grspeed) = decoder::track_and_groundspeed(message, false);
                self.track_source = '\u{2081}';
            }
            2 => {
                // 4 knots units for supersonic
                (self.track, self.grspeed) = decoder::track_and_groundspeed(message, true);
                self.track_source = '\u{2082}';
            }
            3 | 4 => {
                self.heading = decoder::heading(message);
                self.heading_source = '\u{2083}';
                self.altitude_source = '"';
            }
            _ => {}
        }
    }

    pub(super) fn update_from_ext_20_22(&mut self, message: &[u32]) {
        self.altitude_gnss = decoder::altitude_gnss(message);
        self.surveillance_status = decoder::surveillance_status(message);
    }

    pub(super) fn update_from_ext_31(&mut self, message: &[u32]) {
        self.adsb_version = decoder::version(message);
    }
}
