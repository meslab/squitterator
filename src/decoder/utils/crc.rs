use log::debug;

use crate::range_value;

pub(crate) fn get_crc(message: &[u32], df: u32) -> u32 {
    match df {
        0..=15 => crc56(message),
        _ => crc112(message),
    }
}

fn crc112(message: &[u32]) -> u32 {
    let poly = 0xFFFA0480u32;
    let mut data = range_value(message, 1, 32).expect("Cannot set data in crc112.");
    let mut data1 = range_value(message, 33, 64).expect("Cannot set data1 in crc112.");
    let mut data2 = range_value(message, 65, 88)
        .map(|x| x << 8)
        .expect("Cannot set data2 in crc112.");

    for _ in 1..=88 {
        if data & 0x80000000 != 0 {
            data ^= poly;
        }
        data <<= 1;
        if data1 & 0x80000000 != 0 {
            data |= 1;
        }
        data1 <<= 1;
        if data2 & 0x80000000 != 0 {
            data1 |= 1;
        }
        data2 <<= 1;
    }

    debug!("CRC: {:032b}", data);
    data >> 8
}

fn crc56(message: &[u32]) -> u32 {
    let poly = 0xFFFA0480;
    let mut data = range_value(message, 1, 32).expect("Cannot set data in crc56.");

    for _ in 0..32 {
        if (data & 0x80000000) != 0 {
            data ^= poly;
        }
        data <<= 1;
    }

    debug!("CRC: {:032b}", data);
    data >> 8
}

/// Calculate the CRC-24 syndrome of a message.
///
/// This is the remainder of the whole message (data followed by the 24-bit
/// parity/address field) divided by the Mode S CRC-24 generator polynomial
/// (`0xFFF409`, i.e. `x^24 + x^23 + ... + 1`).
///
/// For messages whose parity field holds the pure CRC (DF11, DF17, DF18) a
/// valid frame yields a syndrome of `0`. For messages whose parity field holds
/// the address/parity (AP) overlay (e.g. DF0/4/5/16/20/21/24) the syndrome
/// equals the transmitting ICAO address, so it is not zero even for valid
/// frames.
///
/// # Arguments
///
/// * `message` - The message to calculate the syndrome for
///
/// # Returns
///
/// The 24-bit syndrome of the message
///
pub(crate) fn reminder(message: &[u32]) -> u32 {
    const POLY: u32 = 0x00FF_F409;
    const REG_MASK: u32 = 0x00FF_FFFF;

    let mut reg: u32 = 0;
    for &nibble in message {
        for shift in (0..4).rev() {
            let bit = (nibble >> shift) & 1;
            let top = (reg >> 23) & 1;
            reg = ((reg << 1) | bit) & REG_MASK;
            if top != 0 {
                reg ^= POLY;
            }
        }
    }
    reg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::*;

    #[test]
    fn test_crc112() {
        let squitters = [
            ("8D40621D58C382D690C8AC2863A7", 2646951),
            ("A0001838300000000000007ADA59", 747097),
            ("A800161110010080E6000073D501", 157953),
            ("A800120110010080F600001AFEDD", 5367331),
            ("8D71BC009901DC93C0070788AE4B", 8957515),
            ("8D71BC0060C386EC2FFDEEEBCE0C", 15453708),
            ("8DA7F6429B053D0000000060D7AE", 6346670),
            ("8D4B18FE68BF033F523BF5BAAAEB", 12233451),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = get_message(squitter) {
                let result = crc112(&message);
                assert_eq!(result, *value);
            }
        }
    }

    #[test]
    fn test_crc56() {
        let squitters = [
            ("8D40621D58C382D690C8AC2863A7", 974891),
            ("A0001838300000000000007ADA59", 1050172),
            ("A800161110010080E6000073D501", 1711224),
            ("A800120110010080F600001AFEDD", 2280096),
            ("8D71BC009901DC93C0070788AE4B", 5591534),
            ("8D71BC0060C386EC2FFDEEEBCE0C", 5591534),
            ("8DA7F6429B053D0000000060D7AE", 14119150),
            ("8D4B18FE68BF033F523BF5BAAAEB", 4387321),
            ("02E197B00179C3", 4874557),
            ("02E1983866E711", 1530641),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = get_message(squitter) {
                let result = crc56(&message);
                assert_eq!(result, *value);
            }
        }
    }

    #[test]
    fn test_reminder() {
        // Pure-parity DFs (17) must have a zero syndrome.
        let valid = [
            "8D406B902015A678D4D220AA4BDA",
            "8D40621D58C382D690C8AC2863A7",
            "8D4B18FE68BF033F523BF5BAAAEB",
        ];
        for squitter in valid.iter() {
            if let Some(message) = get_message(squitter) {
                assert_eq!(reminder(&message), 0, "Squitter: {}", squitter);
            }
        }

        // AP-encoded DFs (21) carry the ICAO in the parity overlay, so the
        // syndrome equals the address and is not zero for valid frames.
        let ap = [
            ("A8281D9B10010080F6000058DB4E", 0x39_2F_2C),
            ("A8281B19CE200030A800062E4BFE", 0x44_04_6E),
        ];
        for (squitter, value) in ap.iter() {
            if let Some(message) = get_message(squitter) {
                assert_eq!(reminder(&message), *value, "Squitter: {}", squitter);
            }
        }
    }

    #[test]
    fn test_reminder_rejects_corruption() {
        if let Some(mut message) = get_message("8D406B902015A678D4D220AA4BDA") {
            assert_eq!(reminder(&message), 0);
            message[0] ^= 1;
            assert_ne!(reminder(&message), 0);
        }
    }
}
