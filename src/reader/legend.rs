use super::planes::DisplayFlags;

pub(super) fn print_legend(display_flags: &DisplayFlags) {
    let legend = [
        ("ICAO", "ICAO Address"),
        ("RG", "Registraton Country Code"),
        ("ALT B", "Altitude (Barometric)"),
        ("SQWK", "Squawk"),
        ("CALLSIGN", "Callsign"),
        ("LATITUDE", "Latitude"),
        ("LONGITUDE", "Longitude"),
        ("GSP", "Ground Speed"),
        ("TRK", "Track"),
        ("HDG", "Heading"),
        ("VRATE", "Vertical Rate"),
        ("LC", "Last Contact"),
        ("W", "Wake Turbulence Category"),
    ];

    let legend_speed = [
        ("TAS", "True Air Speed"),
        ("IAS", "Indicated Air Speed"),
        ("MACH", "Mach Number"),
    ];
    let legend_angles = [("RLL", "Roll Angle")];
    let legend_weather = [
        ("TEMP", "Static temperature"),
        ("WND", "Wind Speed"),
        ("WDR", "Wind Direction"),
        ("HUM", "Humidity"),
        ("PRES", "Static pressure"),
        ("TB", "Turbulence"),
    ];

    let legend_extra = [
        ("VX", "Wake Vortex ADS-B Category"),
        ("DF", "Downlink Format"),
        ("TC", "Type Code"),
        ("V", "ASD-B Version"),
        ("S", "Surveillance Status"),
        ("PTH", "Position, Track, Heaging age"),
    ];

    let width = (10, 28);
    let legend_line = legend
        .iter()
        .map(|&(header, description)| {
            format!(
                "{:w0$}: {:w1$}\n",
                header,
                description,
                w0 = width.0,
                w1 = width.1
            )
        })
        .chain(if display_flags.speed() {
            legend_speed
                .iter()
                .map(|&(header, description)| {
                    format!(
                        "{:w0$}: {:w1$}\n",
                        header,
                        description,
                        w0 = width.0,
                        w1 = width.1
                    )
                })
                .collect()
        } else {
            Vec::new()
        })
        .chain(if display_flags.angles() {
            legend_angles
                .iter()
                .map(|&(header, description)| {
                    format!(
                        "{:w0$}: {:w1$}\n",
                        header,
                        description,
                        w0 = width.0,
                        w1 = width.1
                    )
                })
                .collect()
        } else {
            Vec::new()
        })
        .chain(if display_flags.weather() {
            legend_weather
                .iter()
                .map(|&(header, description)| {
                    format!(
                        "{:w0$}: {:w1$}\n",
                        header,
                        description,
                        w0 = width.0,
                        w1 = width.1
                    )
                })
                .collect()
        } else {
            Vec::new()
        })
        .chain(if display_flags.extra() {
            legend_extra
                .iter()
                .map(|&(header, description)| {
                    format!(
                        "{:w0$}: {:w1$}\n",
                        header,
                        description,
                        w0 = width.0,
                        w1 = width.1
                    )
                })
                .collect()
        } else {
            Vec::new()
        })
        .collect::<String>();

    print!("{}", legend_line);
}
