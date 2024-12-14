use super::Plane;
use chrono::Utc;
mod from_bcast;
mod from_ext;
mod from_mode_s;

impl Plane {
    pub fn update(&mut self, message: &[u32], df: u32, relaxed: bool) {
        self.timestamp = Utc::now();
        self.last_df = df;

        self.update_from_bcast(message, df);

        if df == 17 || df == 18 {
            self.update_from_ext(message, df);
        }

        if (relaxed || (self.capability.0 > 3)) && (df == 20 || df == 21) {
            self.update_from_mode_s(message, df, relaxed);
        }
    }
}
