use std::fmt::{self, Debug};

use super::*;

#[derive(Debug)]
pub enum DF {
    SRT(Srt),
    EXT(Ext),
    MDS(Mds),
}

impl fmt::Display for DF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DF::SRT(v) => write!(f, "{}", v),
            DF::EXT(v) => write!(f, "{}", v),
            DF::MDS(v) => write!(f, "{}", v),
        }
    }
}

impl Downlink for DF {
    fn from_message(message: &[u32]) -> Self {
        df(message)
            .map(|df| match df {
                0..=16 => DF::SRT(Srt::from_message(message)),
                17 => DF::EXT(Ext::from_message(message)),
                20 | 21 => DF::MDS(Mds::from_message(message)),
                _ => DF::SRT(Srt::new()),
            })
            .unwrap()
    }

    fn update(&mut self, message: &[u32]) {
        match self {
            DF::SRT(v) => v.update(message),
            DF::EXT(v) => v.update(message),
            DF::MDS(v) => v.update(message),
        }
    }
}

pub trait Downlink {
    fn from_message(message: &[u32]) -> Self;
    fn update(&mut self, message: &[u32]);
}
