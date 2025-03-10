mod calc;
mod crc;
mod format;
mod ma_code;
mod me_code;

pub(crate) use calc::*;
pub(crate) use crc::*;
pub(crate) use format::*;
pub(crate) use ma_code::*;
pub(crate) use me_code::*;

/// Converts a squitter string into a vector of u32 values.
///
/// # Arguments
///
/// * `squitter` - The squitter string to convert.
///
/// # Returns
///
/// * `Option<Vec<u32>>` - An Option vector of u32 values representing the converted squitter string.
pub fn get_message(squitter: &str) -> Option<Vec<u32>> {
    clean_squitter(squitter)
        .filter(|message| matches!(message.len(), 14 | 28))
        .filter(|message| reminder(message) == 0)
}

pub(crate) fn get_hex_message(message: &[u32]) -> String {
    message
        .iter()
        .map(|x| format!("{:X}", x))
        .collect::<Vec<String>>()
        .join("")
}

pub(crate) fn get_message_type(message: &[u32]) -> (u32, u32) {
    ((message[8] << 1) | (message[9] >> 3), message[9] & 7)
}

/// Retrieves the CA (Capability) value from a message.
///
/// # Arguments
///
/// * `message` - The message to extract the CA value from.
///
/// # Returns
///     
/// The CA value.
pub(crate) fn get_capability(message: &[u32]) -> u32 {
    message[1] & 0b0111
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        match get_message(squitter) {
            Some(message) => assert_eq!(
                message,
                vec![
                    8, 13, 4, 0, 6, 2, 1, 13, 5, 8, 12, 3, 8, 2, 13, 6, 9, 0, 12, 8, 10, 12, 2, 8,
                    6, 3, 10, 7
                ]
            ),
            None => panic!("Failed to convert squitter to message"),
        }
    }

    #[test]
    fn test_message_short() {
        let squitter = "02E197B00179C3";
        match get_message(squitter) {
            Some(message) => assert_eq!(message, vec![0, 2, 14, 1, 9, 7, 11, 0, 0, 1, 7, 9, 12, 3]),
            None => panic!("Failed to convert squitter to message"),
        }
    }

    //#[test]
    //fn test_ic() {
    //    let squitter = "8D40621D58C382D690C8AC2863A7";
    //    if let Some(message) = get_message(squitter) {
    //        assert_eq!(ic(&message), 8);
    //    }
    //}

    #[test]
    fn test_get_capability() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = get_message(squitter) {
            assert_eq!(get_capability(&message), 5);
        }
    }

    #[test]
    fn test_message_type() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = get_message(squitter) {
            let (message_type, message_subtype) = get_message_type(&message);
            assert_eq!(message_type, 11);
            assert_eq!(message_subtype, 0);
        }
    }
}
