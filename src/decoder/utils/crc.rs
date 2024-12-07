use std::vec;

use log::debug;

use crate::decoder::range_value;

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

/// Calculate the reminder of the message
///
/// # Arguments
///
/// * `message` - The message to calculate the reminder
///
/// # Returns
///
/// The reminder of the message
///
pub(crate) fn reminder(message: &[u32]) -> u32 {
    let generator = [0b11111111u16, 0b11111010u16, 0b00000100u16, 0b10000000u16];

    let mut bytes = message[0..message.len() - 6]
        .iter()
        .map(|&x| (x & 0b1111) as u8)
        .collect::<Vec<u8>>();
    bytes.append(vec![0; 6].as_mut());

    for i in 0..bytes.len() - 6 {
        for j in 0..8u8 {
            let mask = 0x80 >> j;
            if bytes[i] & mask != 0 {
                bytes[i] ^= (generator[0] >> j) as u8;
                bytes[i + 1] ^= (generator[0] << (8 - j)) as u8 | (generator[1] >> j) as u8;
                bytes[i + 2] ^= (generator[1] << (8 - j)) as u8 | (generator[2] >> j) as u8;
                bytes[i + 3] ^= (generator[2] << (8 - j)) as u8 | (generator[3] >> j) as u8;
            }
        }
    }

    (((bytes[bytes.len() - 3]) as u32) << 16)
        | (((bytes[bytes.len() - 2]) as u32) << 8)
        | (bytes[bytes.len() - 1]) as u32
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
            if let Some(message) = message(squitter) {
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
            if let Some(message) = message(squitter) {
                let result = crc56(&message);
                assert_eq!(result, *value);
            }
        }
    }

    #[test]
    fn test_reminder() {
        let squitters = [
            ("8D406B902015A678D4D220AA4BDA", 0),
            ("A8281D9B10010080F6000058DB4E", 0),
            ("A8281B19CE200030A800062E4BFE", 0),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = message(squitter) {
                let result = reminder(&message);
                assert_eq!(result, *value);
            }
        }
    }
}
