use crate::flag_and_range_value;

/// Calculates the Mode E (ME) code from the given ADS-B message.
///
/// # Arguments
///
/// * `message` - A slice of `u32` representing the ADS-B message.
///
/// # Returns
///
/// An `Option<u16>` representing the calculated ME code. Returns `Some(code)` if the calculation is successful,
/// or `None` if the message is invalid.
pub(crate) fn me_code(message: &[u32]) -> Option<u16> {
    flag_and_range_value(message, 48, 41, 52).map(|(flag, value)| ((value << 2) | flag) as u16)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::message;

    #[test]
    fn test_me_code() {
        let squitters = [
            ("8D40621D58C382D690C8AC2863A7", 12513),
            ("A0001838300000000000007ADA59", 0),
            ("A800161110010080E6000073D501", 65),
            ("A800120110010080F600001AFEDD", 65),
            ("8D71BC009901DC93C0070788AE4B", 117),
            ("8D71BC0060C386EC2FFDEEEBCE0C", 12513),
            ("8DA7F6429B053D0000000060D7AE", 333),
            ("8D4B18FE68BF033F523BF5BAAAEB", 12225),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = message(squitter) {
                let result = me_code(&message);
                assert_eq!(result, Some(*value));
            }
        }
    }

    #[test]
    fn test_me_code_invalid_hex() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            let result = me_code(&message);
            assert_eq!(result, Some(12513));
        }
    }
}
