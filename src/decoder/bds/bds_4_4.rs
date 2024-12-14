use log::debug;

use crate::decoder;

#[derive(Debug)]
pub struct Meteo {
    pub temp: Option<f64>,
    pub wind: Option<(u32, u32)>,
    pub humidity: Option<u32>,
    pub turbulence: Option<u32>,
    pub pressure: Option<u32>,
}

impl Meteo {
    pub fn new() -> Self {
        Meteo {
            temp: None,
            wind: None,
            humidity: None,
            turbulence: None,
            pressure: None,
        }
    }

    pub fn from_data(
        temp: Option<f64>,
        wind: Option<(u32, u32)>,
        humidity: Option<u32>,
        turbulence: Option<u32>,
        pressure: Option<u32>,
    ) -> Self {
        Meteo {
            temp,
            wind,
            humidity,
            turbulence,
            pressure,
        }
    }
}

impl Default for Meteo {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_bds_4_4(message: &[u32]) -> Option<Meteo> {
    let fom = decoder::range_value(message, 33, 36)?;
    if fom > 0b1000
        && decoder::goodflags(message, 37, 38, 55)
        && decoder::goodflags(message, 37, 57, 66)
        && decoder::goodflags(message, 67, 68, 78)
        && decoder::goodflags(message, 79, 80, 81)
        && decoder::goodflags(message, 82, 83, 88)
    {
        let meteo = Meteo::from_data(
            decoder::temperature_4_4(message).filter(|x| (-80.0..=60.0).contains(x)),
            decoder::wind_4_4(message).filter(|x| (0..=300).contains(&x.0)),
            decoder::humidity_4_4(message).filter(|x| (0..=100).contains(x)),
            decoder::turbulence_4_4(message).filter(|x| (0..=15).contains(x)),
            decoder::pressure_4_4(message).filter(|x| (0..=2048).contains(x)),
        );
        debug!(
            "DF:{} F:{:b} {:?}",
            decoder::df(message)?,
            decoder::range_value(message, 33, 36)?,
            meteo
        );

        if meteo.temp.is_some()
            // && meteo.wind.is_some()
            && meteo.humidity.is_some()
            && meteo.turbulence.is_some()
            && meteo.pressure.is_some()
        {
            Some(meteo)
        } else {
            None
        }
    } else {
        None
    }
}
