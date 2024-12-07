use super::{header::DisplayFlags, Plane};
use crate::decoder;
use chrono::Utc;
use std::fmt;

pub trait SimpleDisplay {
    fn simple_display(&self, f: &mut fmt::Formatter, display_flags: &DisplayFlags) -> fmt::Result;
}

impl SimpleDisplay for Plane {
    fn simple_display(&self, f: &mut fmt::Formatter, display_flags: &DisplayFlags) -> fmt::Result {
        write!(f, "{:06X} ", self.icao)?;
        write!(f, "{:2} ", self.reg)?;
        if let Some(squawk) = self.squawk {
            write!(f, "{:04}", squawk)?;
        } else {
            write!(f, "{:4}", "")?;
        }
        if let Some(threat_encounter) = self.threat_encounter {
            write!(f, "{}", threat_encounter)?;
        } else {
            write!(f, " ")?;
        }
        if let Some(w) = decoder::icao_wtc(&self.category) {
            write!(f, "{} ", w)?;
        } else {
            write!(f, "  ")?;
        }
        if let Some(ais) = &self.ais {
            write!(f, "{:8} ", ais)?;
        } else {
            write!(f, "{:8} ", "")?;
        }
        if self.lat != 0.0 && self.lon != 0.0 {
            write!(f, "{:9.5} {:11.5} ", self.lat, self.lon)?;
        } else {
            write!(f, "{:9} {:11} ", "", "")?;
        }
        if let Some(distance_from_observer) = &self.distance_from_observer {
            write!(f, "{:5.1} ", distance_from_observer)?;
        } else {
            write!(f, "{:5} ", "")?;
        }
        if let Some(altitude) = self.altitude {
            write!(f, "{:>5}", altitude)?;
            write!(f, "{}", self.altitude_source)?;
        } else {
            write!(f, "{:5} ", "")?;
        }
        if display_flags.altitude() {
            if let Some(altitude_gnss) = self.altitude_gnss {
                write!(f, "{:>5} ", altitude_gnss)?;
            } else {
                write!(f, "{:5} ", "")?;
            }
            if let Some(selected_altitude) = self.selected_altitude {
                write!(f, "{:>5}{}", selected_altitude, self.target_altitude_source)?;
            } else {
                write!(f, "{:5} ", "")?;
            }
            if let Some(value) = self.barometric_pressure_setting {
                write!(f, "{:>4} ", value)?;
            } else {
                write!(f, "{:4} ", "")?;
            }
        }
        if let Some(vrate) = self.vrate {
            write!(f, "{:>5}", vrate)?;
            write!(f, "{:1}", self.vrate_source)?;
        } else {
            write!(f, "{:6}", "")?;
        }
        if let Some(track) = self.track {
            write!(f, "{:>3.0}", track)?;
            write!(f, "{}", self.track_source)?;
        } else {
            write!(f, "{:4}", "")?;
        }
        if let Some(heading) = self.heading {
            write!(f, "{:>3.0}", heading)?;
            write!(f, "{:}", self.heading_source)?;
        } else {
            write!(f, "{:4}", "")?;
        }
        if let Some(grspeed) = self.grspeed {
            write!(f, "{:>3.0} ", grspeed)?;
        } else {
            write!(f, "{:3} ", "")?;
        }
        if display_flags.speed() {
            if let Some(tas) = self.true_airspeed {
                write!(f, "{:>3} ", tas)?;
            } else {
                write!(f, "{:3} ", "")?;
            }
            if let Some(ias) = self.indicated_airspeed {
                write!(f, "{:>3} ", ias)?;
            } else {
                write!(f, "{:3} ", "")?;
            }
            if let Some(mn) = self.mach_number {
                write!(f, "{:>4.2} ", mn)?;
            } else {
                write!(f, "{:4} ", "")?;
            }
        }
        if display_flags.angles() {
            if let Some(roll_angle) = self.roll_angle {
                write!(f, "{:>3} ", roll_angle)?;
            } else {
                write!(f, "{:3} ", "")?;
            }
            if let Some(track_angle_rate) = self.track_angle_rate {
                write!(f, "{:>3} ", track_angle_rate)?;
            } else {
                write!(f, "{:3} ", "")?;
            }
        }
        if display_flags.weather() {
            if let Some(temperature) = self.temperature {
                write!(f, "{:>5.1} ", temperature)?;
            } else {
                write!(f, "{:5} ", "")?;
            }
            if let Some(wind) = self.wind {
                write!(f, "{:>3} ", wind.0)?;
                write!(f, "{:>3} ", wind.1)?;
            } else {
                write!(f, "{:7} ", "")?;
            }
            if let Some(humidity) = self.humidity {
                write!(f, "{:>3} ", humidity)?;
            } else {
                write!(f, "{:3} ", "")?;
            }
            if let Some(pressure) = self.pressure {
                write!(f, "{:>4} ", pressure)?;
            } else {
                write!(f, "{:4} ", "")?;
            }
            if let Some(turbulence) = self.turbulence {
                write!(f, "{:>2} ", turbulence)?;
            } else {
                write!(f, "{:2} ", "")?;
            }
        }
        if display_flags.extra() {
            write!(f, "{}{} ", self.category.0, self.category.1)?;
            if self.last_df != 0 {
                write!(f, "{:>2} ", self.last_df)?;
            } else {
                write!(f, "{:2} ", "")?;
            }
            if self.last_type_code != 0 {
                write!(f, "{:>2} ", self.last_type_code)?;
            } else {
                write!(f, "{:2} ", "")?;
            }
            if let Some(version) = self.adsb_version {
                write!(f, "{:1} ", version)?;
            } else {
                write!(f, "{:1} ", "")?;
            }
            write!(f, "{} ", self.surveillance_status)?;
            if let Some(position_timestamp) = self.position_timestamp {
                write!(
                    f,
                    "{:X}",
                    Utc::now()
                        .signed_duration_since(position_timestamp)
                        .num_seconds()
                        / 10
                        % 16
                )?;
            } else {
                write!(f, " ")?;
            }
            if let Some(track_timestamp) = self.track_timestamp {
                write!(
                    f,
                    "{:X}",
                    Utc::now()
                        .signed_duration_since(track_timestamp)
                        .num_seconds()
                        / 10
                        % 16
                )?;
            } else {
                write!(f, " ")?;
            }
            if let Some(heading_timestamp) = self.heading_timestamp {
                write!(
                    f,
                    "{:X} ",
                    Utc::now()
                        .signed_duration_since(heading_timestamp)
                        .num_seconds()
                        / 10
                        % 16
                )?;
            } else {
                write!(f, "  ")?;
            }
        }
        write!(
            f,
            "{:>2}",
            Utc::now()
                .signed_duration_since(self.timestamp)
                .num_seconds()
        )
    }
}

pub struct SimpleDisplayWrapper<'a, T: SimpleDisplay>(&'a T, &'a DisplayFlags);

impl<T: SimpleDisplay> fmt::Display for SimpleDisplayWrapper<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.simple_display(f, self.1)
    }
}

pub fn format_simple_display<T: SimpleDisplay>(item: &T, display_flags: &DisplayFlags) -> String {
    format!("{}", SimpleDisplayWrapper(item, display_flags))
}
