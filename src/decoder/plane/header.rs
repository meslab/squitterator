pub struct LegendHeaders {
    pub header: String,
    pub separator: String,
}

impl LegendHeaders {
    pub fn from_display_flags(display_flags: &DisplayFlags) -> Self {
        let mut headers = vec![
            ("ICAO", 6),
            ("RG", 2),
            ("SQWK", 4),
            ("W", 1),
            ("CALLSIGN", 8),
            ("LATITUDE", 9),
            ("LONGITUDE", 11),
            ("DIST", 5),
            ("ALT B", 5),
        ];

        if display_flags.altitude() {
            headers.extend([("ALT G", 5), ("ALT S", 5), ("BARO", 4)]);
        }

        headers.extend([("VRATE", 5), ("TRK", 3), ("HDG", 3), ("GSP", 3)]);

        if display_flags.speed() {
            headers.extend([("TAS", 3), ("IAS", 3), ("MACH", 4)]);
        }
        if display_flags.angles() {
            headers.extend([("RLL", 3), ("TAR", 3)]);
        }
        if display_flags.weather() {
            headers.extend([
                ("TEMP", 5),
                ("WND", 3),
                ("WDR", 3),
                ("HUM", 3),
                ("PRES", 4),
                ("TB", 2),
            ]);
        }
        if display_flags.extra() {
            headers.extend([
                ("VX", 2),
                ("DF", 2),
                ("TC", 2),
                ("V", 1),
                ("S", 1),
                ("PTH", 3),
            ]);
        }

        let (header_line, separator_line) = headers.iter().fold(
            (String::new(), String::new()),
            |(mut header_line, mut separator_line), &(header, width)| {
                use std::fmt::Write;
                write!(&mut header_line, "{:>width$} ", header, width = width)
                    .expect("Cannot set header.");
                write!(&mut separator_line, "{:-<width$} ", "", width = width)
                    .expect("Cannot set separator.");
                (header_line, separator_line)
            },
        );

        LegendHeaders {
            header: header_line + "LC\n",
            separator: separator_line + "--\n",
        }
    }

    pub fn print_header(&self) {
        print!("{}", self.header);
    }

    pub fn print_separator(&self) {
        print!("{}", self.separator);
    }
}

pub struct DisplayFlags {
    pub bits: u8,
}

impl DisplayFlags {
    pub fn new(weather: bool, angles: bool, speed: bool, altitude: bool, extra: bool) -> Self {
        let mut bits = 0u8;
        if weather {
            bits |= 1 << 0;
        }
        if angles {
            bits |= 1 << 1;
        }
        if speed {
            bits |= 1 << 2;
        }
        if altitude {
            bits |= 1 << 3;
        }
        if extra {
            bits |= 1 << 4;
        }
        DisplayFlags { bits }
    }

    pub fn weather(&self) -> bool {
        self.bits & (1 << 0) != 0
    }
    pub fn angles(&self) -> bool {
        self.bits & (1 << 1) != 0
    }
    pub fn speed(&self) -> bool {
        self.bits & (1 << 2) != 0
    }
    pub fn altitude(&self) -> bool {
        self.bits & (1 << 3) != 0
    }
    pub fn extra(&self) -> bool {
        self.bits & (1 << 4) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        let display_flags = DisplayFlags::new(false, false, false, false, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP LC\n"
        )
    }

    #[test]
    fn test_weather_0() {
        let display_flags = DisplayFlags::new(true, false, false, false, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP  TEMP WND WDR HUM PRES TB LC\n"
        )
    }

    #[test]
    fn test_weather_1() {
        let display_flags = DisplayFlags::new(true, true, false, false, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
           "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP RLL TAR  TEMP WND WDR HUM PRES TB LC\n"
        )
    }

    #[test]
    fn test_weather_2() {
        let display_flags = DisplayFlags::new(true, true, true, false, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
           "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP TAS IAS MACH RLL TAR  TEMP WND WDR HUM PRES TB LC\n"
        )
    }

    #[test]
    fn test_weather_3() {
        let display_flags = DisplayFlags::new(true, true, true, true, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
           "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH RLL TAR  TEMP WND WDR HUM PRES TB LC\n"
        )
    }

    #[test]
    fn test_weather_4() {
        let display_flags = DisplayFlags::new(true, true, true, true, true);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
           "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH RLL TAR  TEMP WND WDR HUM PRES TB VX DF TC V S PTH LC\n"
        )
    }

    #[test]
    fn test_angles_0() {
        let display_flags = DisplayFlags::new(false, true, false, false, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP RLL TAR LC\n"
        )
    }

    #[test]
    fn test_angles_1() {
        let display_flags = DisplayFlags::new(false, true, true, false, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP TAS IAS MACH RLL TAR LC\n"
        )
    }

    #[test]
    fn test_angles_2() {
        let display_flags = DisplayFlags::new(false, true, true, true, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH RLL TAR LC\n"
        )
    }

    #[test]
    fn test_angles_3() {
        let display_flags = DisplayFlags::new(false, true, true, true, true);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH RLL TAR VX DF TC V S PTH LC\n"
        )
    }

    #[test]
    fn test_speed_0() {
        let display_flags = DisplayFlags::new(false, false, true, false, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP TAS IAS MACH LC\n"
        )
    }

    #[test]
    fn test_speed_1() {
        let display_flags = DisplayFlags::new(false, false, true, true, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
             "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH LC\n"
        )
    }

    #[test]
    fn test_speed_2() {
        let display_flags = DisplayFlags::new(false, false, true, true, true);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH VX DF TC V S PTH LC\n"
        )
    }

    #[test]
    fn test_altitude_0() {
        let display_flags = DisplayFlags::new(false, false, false, true, false);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP LC\n"
        )
    }

    #[test]
    fn test_altitude_1() {
        let display_flags = DisplayFlags::new(false, false, false, true, true);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP VX DF TC V S PTH LC\n"
        )
    }

    #[test]
    fn test_extra_0() {
        let display_flags = DisplayFlags::new(false, false, false, false, true);
        let headers = LegendHeaders::from_display_flags(&display_flags);
        assert_eq!(
            headers.header,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP VX DF TC V S PTH LC\n"
        )
    }
}
