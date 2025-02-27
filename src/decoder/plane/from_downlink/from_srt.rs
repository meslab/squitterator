use crate::decoder::{Plane, Srt, plane::from_downlink::UpdateFromDownlink};

impl UpdateFromDownlink<Srt> for Plane {
    fn update_from_downlink(&mut self, dl: &Srt) {
        if dl.icao.is_some() {
            if dl.df == Some(4) && dl.altitude.is_some() {
                self.altitude = dl.altitude;
                self.altitude_source = ' ';
            }
            if dl.df == Some(5) && dl.squawk.is_some() {
                self.squawk = dl.squawk;
            }
            if dl.df == Some(11) {
                if let Some(v) = dl.capability {
                    self.capability.0 = v;
                }
            }
        }
    }
}
