use super::Ext;
use crate::decoder;

impl Ext {
    fn update_mt_1_4(&mut self, message: &[u32]) {
        self.ais = decoder::ais(message);
        self.category = Some(self.message_type);
    }

    fn update_mt_5_18(&mut self, message: &[u32], df: u32) {
        self.cpr = decoder::cpr(message);
        match self.message_type.0 {
            5..=8 => {
                self.ground_movement = decoder::ground_movement(message);
                self.track = decoder::ground_track(message);
                self.track_source = Some('\u{2070}');
                self.altitude_source = Some('\u{2070}');
            }
            9..=18 => {
                self.altitude = decoder::altitude(message, df);
                self.surveillance_status = Some(decoder::surveillance_status(message));
            }
            _ => {}
        }
    }

    fn update_mt_19(&mut self, message: &[u32]) {
        self.vrate = decoder::vertical_rate(message);
        self.altitude_delta = decoder::altitude_delta(message);
        match self.message_type.1 {
            1 => {
                (self.track, self.grspeed) = decoder::track_and_groundspeed(message, false);
                self.track_source = Some('\u{2081}');
            }
            2 => {
                (self.track, self.grspeed) = decoder::track_and_groundspeed(message, true);
                self.track_source = Some('\u{2082}');
            }
            3 | 4 => {
                self.heading = decoder::heading(message);
                self.heading_source = Some('\u{2083}');
            }
            _ => {}
        }
    }

    fn update_mt_20_22(&mut self, message: &[u32]) {
        self.altitude_gnss = decoder::altitude_gnss(message);
        self.surveillance_status = Some(decoder::surveillance_status(message));
    }

    fn update_mt_31(&mut self, message: &[u32]) {
        self.adsb_version = decoder::version(message);
    }
}

impl decoder::Downlink for Ext {
    fn from_get_message(message: &[u32]) -> Result<Self, &str> {
        let mut dl = Ext::new();
        dl.update(message);
        Ok(dl)
    }

    fn update(&mut self, message: &[u32]) {
        if let Some(df) = decoder::get_downlink_format(message) {
            self.df = Some(df);
            self.icao = decoder::icao(message, df);
            self.capability = decoder::get_capability(message);
            self.message_type = decoder::get_message_type(message);
            match self.message_type.0 {
                1..=4 => {
                    self.update_mt_1_4(message);
                }
                5..=18 => {
                    self.update_mt_5_18(message, df);
                }
                19 => {
                    self.update_mt_19(message);
                }
                20..=22 => {
                    self.update_mt_20_22(message);
                }
                31 => {
                    self.update_mt_31(message);
                }
                _ => {}
            }
        };
    }

    fn icao(&self) -> Option<u32> {
        self.icao
    }
}
