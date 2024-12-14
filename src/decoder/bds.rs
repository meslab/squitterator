mod bds_1_7;
mod bds_4_0;
mod bds_4_4;
mod bds_4_5;
mod bds_5_0;
mod bds_6_0;

pub(crate) use bds_1_7::*;
pub(crate) use bds_4_0::*;
pub(crate) use bds_4_4::*;
pub(crate) use bds_4_5::*;
pub(crate) use bds_5_0::*;
pub(crate) use bds_6_0::*;

use super::{flag_and_range_value, range_value};

/// Retrieves the BDS values from a message.
///
/// # Arguments
///
/// * `message` - The message to extract the BDS values from.
///
/// # Returns
///
/// A tuple containing the BDS1 and BDS2 values.
pub fn bds(message: &[u32]) -> (u32, u32) {
    if let (1, 0) = (message[8] & 0xF, message[9] & 0xF) {
        if message[10] & 0x7 == 0 && message[11] & 0xC == 0 {
            return (1, 0);
        }
    };

    if let (2, 0) = (message[8] & 0xF, message[9] & 0xF) {
        return (2, 0);
    };

    if let (3, 0) = (message[8] & 0xF, message[9] & 0xF) {
        if let Some(value) = range_value(message, 48, 54) {
            if (message[15] & 0b1100) != 0b1100 && value < 48 {
                return (3, 0);
            }
        }
    };

    (0, 0)
}

pub(crate) fn goodflags(message: &[u32], flag: u32, sb: u32, eb: u32) -> bool {
    match flag_and_range_value(message, flag, sb, eb) {
        Some((flag, result)) => match flag {
            0 => false,
            _ => !matches!(result, 0),
        },
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::message;

    #[test]
    fn test_bds() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            assert_eq!(bds(&message), (0, 0));
        }
    }

    #[test]
    fn test_bds_21() {
        let squitter = "A8000096300000000000007F5EBC";
        if let Some(message) = message(squitter) {
            assert_eq!(bds(&message), (3, 0));
        }
    }

    #[test]
    fn test_badflags() {
        let messages = [
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                32,
                33,
                37,
                (1, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                36,
                (1, 8),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                37,
                (1, 16),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 8, 0, 0, 0, 0],
                32,
                33,
                37,
                (1, 17),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 8, 0, 0, 0, 0],
                32,
                34,
                37,
                (1, 1),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                38,
                (1, 32),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                39,
                (1, 64),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 0, 0],
                32,
                33,
                40,
                (1, 128),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0],
                32,
                34,
                46,
                (1, 0b1_0000_0000_0000),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0],
                33,
                34,
                41,
                (1, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
                34,
                35,
                43,
                (1, 0),
            ),
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
                35,
                36,
                44,
                (1, 0),
            ),
        ];
        for (message, f, s, e, result) in messages {
            assert_eq!(
                flag_and_range_value(&message, f, s, e),
                Some(result),
                "F:{} S:{} E:{} L:{}",
                f,
                s,
                e,
                e - s + 1
            );
        }
    }
}
