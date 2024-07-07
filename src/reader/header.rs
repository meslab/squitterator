pub(super) fn print_header(
    weather: bool,
    angles: bool,
    speed: bool,
    altitude: bool,
    extra: bool,
    header: bool,
) {
    let (header_line, separator_line) = get_header(weather, angles, speed, altitude, extra);

    if header {
        print!("{}{}", header_line, separator_line);
    } else {
        print!("{}", separator_line);
    }
}

fn get_header(
    weather: bool,
    angles: bool,
    speed: bool,
    altitude: bool,
    extra: bool,
) -> (String, String) {
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

    if altitude {
        headers.extend([("ALT G", 5), ("ALT S", 5), ("BARO", 4)]);
    }

    headers.extend([("VRATE", 5), ("TRK", 3), ("HDG", 3), ("GSP", 3)]);

    if speed {
        headers.extend([("TAS", 3), ("IAS", 3), ("MACH", 4)]);
    }
    if angles {
        headers.extend([("RLL", 3), ("TAR", 3)]);
    }
    if weather {
        headers.extend([
            ("TEMP", 5),
            ("WND", 3),
            ("WDR", 3),
            ("HUM", 3),
            ("PRES", 4),
            ("TB", 2),
        ]);
    }
    if extra {
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
            write!(&mut header_line, "{:>width$} ", header, width = width).unwrap();
            write!(&mut separator_line, "{:-<width$} ", "", width = width).unwrap();
            (header_line, separator_line)
        },
    );

    let header_line = header_line + "LC\n";
    let separator_line = separator_line + "--\n";

    (header_line, separator_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        let (weather, angles, speed, altitude, extra) = (false, false, false, false, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP LC\n"
        )
    }

    #[test]
    fn test_weather_0() {
        let (weather, angles, speed, altitude, extra) = (true, false, false, false, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP  TEMP WND WDR HUM PRES TB LC\n"
        )
    }

    #[test]
    fn test_weather_1() {
        let (weather, angles, speed, altitude, extra) = (true, true, false, false, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
           "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP RLL TAR  TEMP WND WDR HUM PRES TB LC\n"
        )
    }

    #[test]
    fn test_weather_2() {
        let (weather, angles, speed, altitude, extra) = (true, true, true, false, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
           "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP TAS IAS MACH RLL TAR  TEMP WND WDR HUM PRES TB LC\n"
        )
    }

    #[test]
    fn test_weather_3() {
        let (weather, angles, speed, altitude, extra) = (true, true, true, true, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
           "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH RLL TAR  TEMP WND WDR HUM PRES TB LC\n"
        )
    }

    #[test]
    fn test_weather_4() {
        let (weather, angles, speed, altitude, extra) = (true, true, true, true, true);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
           "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH RLL TAR  TEMP WND WDR HUM PRES TB VX DF TC V S PTH LC\n"
        )
    }

    #[test]
    fn test_angles_0() {
        let (weather, angles, speed, altitude, extra) = (false, true, false, false, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP RLL TAR LC\n"
        )
    }

    #[test]
    fn test_angles_1() {
        let (weather, angles, speed, altitude, extra) = (false, true, true, false, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP TAS IAS MACH RLL TAR LC\n"
        )
    }

    #[test]
    fn test_angles_2() {
        let (weather, angles, speed, altitude, extra) = (false, true, true, true, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH RLL TAR LC\n"
        )
    }

    #[test]
    fn test_angles_3() {
        let (weather, angles, speed, altitude, extra) = (false, true, true, true, true);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH RLL TAR VX DF TC V S PTH LC\n"
        )
    }

    #[test]
    fn test_speed_0() {
        let (weather, angles, speed, altitude, extra) = (false, false, true, false, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP TAS IAS MACH LC\n"
        )
    }

    #[test]
    fn test_speed_1() {
        let (weather, angles, speed, altitude, extra) = (false, false, true, true, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
             "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH LC\n"
        )
    }

    #[test]
    fn test_speed_2() {
        let (weather, angles, speed, altitude, extra) = (false, false, true, true, true);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP TAS IAS MACH VX DF TC V S PTH LC\n"
        )
    }

    #[test]
    fn test_altitude_0() {
        let (weather, angles, speed, altitude, extra) = (false, false, false, true, false);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP LC\n"
        )
    }

    #[test]
    fn test_altitude_1() {
        let (weather, angles, speed, altitude, extra) = (false, false, false, true, true);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B ALT G ALT S BARO VRATE TRK HDG GSP VX DF TC V S PTH LC\n"
        )
    }

    #[test]
    fn test_extra_0() {
        let (weather, angles, speed, altitude, extra) = (false, false, false, false, true);
        let headers = get_header(weather, angles, speed, altitude, extra);
        assert_eq!(
            headers.0,
            "  ICAO RG SQWK W CALLSIGN  LATITUDE   LONGITUDE  DIST ALT B VRATE TRK HDG GSP VX DF TC V S PTH LC\n"
        )
    }
}
