mod from_ext;
mod from_mds;
mod from_srt;

use crate::decoder::{DF, Plane};
use chrono::Utc;

pub trait UpdateFromDownlink<T> {
    fn update_from_downlink(&mut self, dl: &T);
}

impl UpdateFromDownlink<DF> for Plane {
    fn update_from_downlink(&mut self, dl: &DF) {
        self.timestamp = Utc::now();
        if let Some(df) = match dl {
            DF::SRT(v) => v.df,
            DF::EXT(v) => v.df,
            DF::MDS(v) => v.df,
        } {
            self.last_df = df;
        }

        match dl {
            DF::SRT(v) => self.update_from_downlink(v),
            DF::EXT(v) => self.update_from_downlink(v),
            DF::MDS(v) => self.update_from_downlink(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::{Downlink, get_message};
    use chrono::Duration;

    fn downlink(squitter: &str) -> DF {
        let message = get_message(squitter).expect("Failed to convert squitter");
        DF::from_message(&message).expect("Failed to build downlink")
    }

    #[test]
    fn test_update_from_downlink_refreshes_timestamp_and_df() {
        let mut plane = Plane::new();
        plane.icao = 0x40621D;
        plane.timestamp = Utc::now() - Duration::seconds(120);
        plane.last_df = 0;
        let old_timestamp = plane.timestamp;

        plane.update_from_downlink(&downlink("8D40621D58C382D690C8AC2863A7"));

        assert!(plane.timestamp > old_timestamp);
        assert!((Utc::now() - plane.timestamp).num_seconds() < 5);
        assert_eq!(plane.last_df, 17);
    }
}
