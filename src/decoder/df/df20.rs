use std::fmt::{self, Display};

use crate::decoder;

pub struct Df20 {
    pub icao: Option<u32>,
    pub altitude: Option<u32>,
}

impl Default for Df20 {
    fn default() -> Self {
        Self::new()
    }
}

impl Df20 {
    pub fn new() -> Self {
        Df20 {
            icao: None,
            altitude: None,
        }
    }

    pub fn from_message(message: &[u32]) -> Self {
        if let Some(df) = super::df(message) {
            Df20 {
                icao: decoder::icao(message, df),
                altitude: None,
            }
        } else {
            Df20::new()
        }
    }
}

impl Display for Df20 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(v) = self.icao {
            write!(f, "{:X},", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.altitude {
            write!(f, "{},", v)?
        } else {
            write!(f, ",")?
        }
        write!(f, "")
    }
}
