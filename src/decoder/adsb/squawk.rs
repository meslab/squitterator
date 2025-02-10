use crate::decoder::ma_code;

/// Converts a Mode S squawk code from an ADS-B message into a decimal value.
///
/// # Arguments
///
/// * `message` - A slice of `u32` representing the ADS-B message.
///
/// # Returns
///
/// * `Some(u32)` - The decimal value of the squawk code if it exists in the message.
/// * `None` - If the squawk code does not exist in the message.
///
pub(crate) fn squawk(message: &[u32]) -> Option<u32> {
    ma_code(message).map(|code| {
        ((((code >> 8) & 1) << 2) | (((code >> 10) & 1) << 1) | ((code >> 12) & 1)) as u32 * 1000
            + ((((code >> 3) & 1) << 2) | (((code >> 5) & 1) << 1) | ((code >> 7) & 1)) as u32 * 100
            + ((((code >> 9) & 1) << 2) | (((code >> 11) & 1) << 1) | ((code >> 13) & 1)) as u32
                * 10
            + ((((code >> 2) & 1) << 2) | (((code >> 4) & 1) << 1) | ((code >> 6) & 1)) as u32
    })
}

#[cfg(test)]
mod tests {
    use crate::decoder::{get_message, squawk};

    #[test]
    fn test_squawk() {
        let squitters = [
            ("A8000F8FC6500030A40000318121", 7666),
            ("A8000F8FE61A2F346017FF0D7928", 7666),
            ("A8000EABEA2A4F34E02400EE982C", 7724),
            ("A8000EAB300000000000002A0FDD", 7724),
            ("A800189AC6500030A400005830F8", 5611),
            ("A800189A805CE93F8004F6F2BCA4", 5611),
            ("2800189A8E0F41", 5611),
            ("2800189F714598", 5617),
            ("A800189FEBCA3B357FD400CCA6D6", 5617),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = get_message(squitter) {
                let result = squawk(&message);
                assert_eq!(result, Some(*value));
            }
        }
    }
}
