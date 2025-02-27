mod from_ext;
mod from_mds;
mod from_srt;

use crate::decoder::{DF, Plane};

pub trait UpdateFromDownlink<T> {
    fn update_from_downlink(&mut self, dl: &T);
}

impl UpdateFromDownlink<DF> for Plane {
    fn update_from_downlink(&mut self, dl: &DF) {
        match dl {
            DF::SRT(v) => self.update_from_downlink(v),
            DF::EXT(v) => self.update_from_downlink(v),
            DF::MDS(v) => self.update_from_downlink(v),
        }
    }
}
