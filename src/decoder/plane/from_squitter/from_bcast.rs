use super::Plane;
use crate::decoder;

impl Plane {
    pub(super) fn update_from_bcast(&mut self, message: &[u32], df: u32) {
        if df == 4 || df == 20 {
            self.altitude = decoder::altitude(message, df);
            self.altitude_source = ' ';
        }
        if df == 5 || df == 21 {
            self.squawk = decoder::squawk(message);
        }

        if df == 11 || df == 17 {
            self.capability.0 = decoder::get_capability(message);
        }
    }
}
