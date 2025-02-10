use crate::decoder;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Srt {
    pub df: Option<u32>,
    pub icao: Option<u32>,
    pub squawk: Option<u32>,
    pub capability: Option<u32>,
    pub altitude: Option<u32>,
}

impl Default for Srt {
    fn default() -> Self {
        Self::new()
    }
}

impl Srt {
    pub fn new() -> Self {
        Srt {
            df: None,
            icao: None,
            squawk: None,
            capability: None,
            altitude: None,
        }
    }
}

impl Display for Srt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(v) = self.df {
            write!(f, "DF{:02}", v)?
        } else {
            write!(f, "")?
        }
        if let Some(v) = self.icao {
            write!(f, ",{:X}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.squawk {
            writeln!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.capability {
            write!(f, ",{}", v)?
        } else {
            write!(f, ",")?
        }
        if let Some(v) = self.altitude {
            writeln!(f, ",{}", v)
        } else {
            writeln!(f, ",")
        }
    }
}

impl decoder::Downlink for Srt {
    fn from_get_message(message: &[u32]) -> Result<Self, &str> {
        let mut dl = Srt::new();
        dl.update(message);
        Ok(dl)
    }

    fn update(&mut self, message: &[u32]) {
        if let Some(df) = decoder::df(message) {
            self.df = Some(df);
            self.icao = decoder::icao(message, df);
            match df {
                4 => {
                    self.altitude = decoder::altitude(message, df);
                }
                5 => {
                    self.squawk = decoder::squawk(message);
                }
                11 => {
                    self.capability = Some(decoder::get_capability(message));
                }
                _ => {}
            }
        }
    }

    fn icao(&self) -> Option<u32> {
        self.icao
    }
}
