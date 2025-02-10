use crate::decoder::get_crc;

/// Calculates the ICAO address based on the given ADS-B message and downlink format (DF).
///
/// # Arguments
///
/// * `message` - The ADS-B message as an array of 32-bit unsigned integers.
/// * `df` - The downlink format (DF) of the ADS-B message.
///
/// # Returns
///
/// The calculated ICAO address as a 32-bit unsigned integer.
///
/// # Examples
///
/// ```
/// use squitterator::{get_message, get_downlink_format, get_icao};
/// if let Some(message) = get_message("A0001838300000000000007ADA59") {
///     if let Some(df) = get_downlink_format(&message) {
///         if let Some(icao_address) = get_icao(&message, df) {
///             assert_eq!(icao_address, 7453696);
///         }
///     }
/// }
pub fn get_icao(message: &[u32], df: u32) -> Option<u32> {
    match df {
        0 | 4 | 5 | 16 | 20 | 21 => {
            let len = (message.len() * 4) as u32;
            crate::range_value(message, len - 23, len)
                .map(|result| result ^ get_crc(message, df))
                .filter(|&f| f != 0)
        }
        _ => crate::range_value(message, 9, 32).filter(|&f| f != 0),
    }
}

/// Calculates the Wake Turbulence Category (WTC) based on the given VDL Mode 2 Code (VC).
/// The WTC is used to determine the separation minima between aircraft.
///
/// # Arguments
///
/// * `vc` - The VDL Mode 2 Code (VC) as a tuple of two 32-bit unsigned integers.
///
/// # Returns
///
/// The calculated Wake Turbulence Category (WTC) as a character.
///
pub(crate) fn get_wake_turbulence_category(vc: &(u32, u32)) -> Option<char> {
    match vc {
        (4, 1) => Some('L'),
        (4, 2) => Some('S'),
        (4, 3) => Some('M'),
        (4, 4) => Some('H'),
        (4, 5) => Some('J'),
        (4, 7) => Some('R'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::{get_downlink_format, get_icao, get_message};

    #[test]
    fn test_icao() {
        let squitters = [
            ("8D40621D58C382D690C8AC2863A7", 4219421),
            ("A0001838300000000000007ADA59", 7453696),
            ("A800161110010080E6000073D501", 7453696),
            ("A800120110010080F600001AFEDD", 4921598),
            ("8D71BC009901DC93C0070788AE4B", 7453696),
            ("8D71BC0060C386EC2FFDEEEBCE0C", 7453696),
            ("8DA7F6429B053D0000000060D7AE", 11007554),
            ("8D4B18FE68BF033F523BF5BAAAEB", 4921598),
            ("28001A1B1F0706", 5023854),
            ("8D4CA86E58B15398DA1B2834CF37", 5023854),
            ("A425B00A580840092F81204A5821", 11188242),
            ("A020100A10020A80F000004F24AF", 12612818),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = get_message(squitter) {
                if let Some(df) = get_downlink_format(&message) {
                    if let Some(result) = get_icao(&message, df) {
                        assert_eq!(result, *value, "Squitter: {} ICAO:{:X}", squitter, result);
                    }
                }
            }
        }
    }

    #[test]
    fn test_get_wake_turbulence_category() {
        let vcs = [
            ((4, 1), 'L'),
            ((4, 2), 'S'),
            ((4, 3), 'M'),
            ((4, 4), 'H'),
            ((4, 5), 'J'),
            ((4, 7), 'R'),
        ];

        for (vc, value) in vcs.iter() {
            if let Some(result) = crate::decoder::get_wake_turbulence_category(vc) {
                assert_eq!(result, *value, "VC: {:?}", vc);
            }
        }
    }

    #[test]
    fn test_get_wake_turbulence_category_none() {
        let vcs = [(4, 0), (4, 6)];

        for vc in vcs.iter() {
            assert_eq!(
                crate::decoder::get_wake_turbulence_category(vc),
                None,
                "VC: {:?}",
                vc
            );
        }
    }
}
