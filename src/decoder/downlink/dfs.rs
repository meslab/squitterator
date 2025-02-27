use std::{
    fmt::{self, Debug, Display},
    fs::File,
    io::Write,
    sync::Mutex,
};

use log::debug;

use super::{Ext, Mds, Srt, get_downlink_format};

#[derive(Debug)]
pub enum DF {
    SRT(Srt),
    EXT(Ext),
    MDS(Mds),
}

#[derive(Debug)]
pub struct DownlinkFrame<T: Downlink> {
    downlink: T,
}

impl<T: Downlink + Display> Display for DownlinkFrame<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.downlink)
    }
}

impl Display for DF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DF::SRT(v) => write!(f, "{}", v),
            DF::EXT(v) => write!(f, "{}", v),
            DF::MDS(v) => write!(f, "{}", v),
        }
    }
}

impl DF {
    pub fn log(&self, downlink_error_log_file: &Mutex<File>) -> Result<(), std::io::Error> {
        let mut downlink_error_log_file = downlink_error_log_file
            .lock()
            .expect("Cannot open downlink error log file.");
        write!(downlink_error_log_file, "{}", self)?;
        debug!("Writing to {:?}", &downlink_error_log_file);
        Ok(())
    }
}

impl Downlink for DF {
    fn from_message(message: &[u32]) -> Result<Self, &str> {
        match get_downlink_format(message) {
            Some(value) => {
                let dl = match value {
                    0..=16 => DF::SRT(Srt::from_message(message)?),
                    17 => DF::EXT(Ext::from_message(message)?),
                    20 | 21 => DF::MDS(Mds::from_message(message)?),
                    _ => DF::SRT(Srt::new()),
                };
                Ok(dl)
            }
            None => Err("cant get df value"),
        }
    }

    fn update(&mut self, message: &[u32]) {
        match self {
            DF::SRT(v) => v.update(message),
            DF::EXT(v) => v.update(message),
            DF::MDS(v) => v.update(message),
        }
    }

    fn icao(&self) -> Option<u32> {
        match self {
            DF::SRT(v) => v.icao,
            DF::EXT(v) => v.icao,
            DF::MDS(v) => v.icao,
        }
    }
}

pub trait Downlink: Sized {
    fn from_message(message: &[u32]) -> Result<Self, &str>;
    fn update(&mut self, message: &[u32]);
    fn icao(&self) -> Option<u32>;
}
