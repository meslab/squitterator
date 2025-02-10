mod delta;
mod gnss;
mod graytobin;

pub(crate) use delta::*;
pub(crate) use gnss::*;

use graytobin::graytobin;
use log::info;

use crate::decoder::{ma_code, me_code};

pub fn altitude(message: &[u32], df: u32) -> Option<u32> {
    let code = match df {
        17 => me_code(message),
        _ => ma_code(message),
    };

    altitude_value(message, code).and_then(|a| {
        if a < 100000 {
            Some(a)
        } else {
            info!(
                "DF:{} C:{:b} M:{} ALT:{}",
                df,
                code?,
                crate::decoder::get_hex_message(message),
                a
            );
            None
        }
    })
}

fn altitude_value(message: &[u32], code: Option<u16>) -> Option<u32> {
    match code {
        Some(code) => match code & 0b10 {
            0 => match code & 1 {
                0 => {
                    let (high, low) = graytobin(message);
                    let value = high * 500 + low * 100;
                    match value {
                        1200.. => Some(high * 500 + low * 100 - 1200),
                        _ => None,
                    }
                }
                _ => Some((((code >> 7) << 4) | ((code >> 2) & 0b1111)) as u32 * 25 - 1000),
            },
            _ => Some(
                ((((code >> 7) << 4) & 0b11111110000 | (code >> 2) & 0b1111) as f32 * 0.31) as u32,
            ),
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder;

    #[test]
    fn test_alt() {
        if let Some(message) = decoder::get_message("A8281200200464B3CF7820CD194C") {
            let df = decoder::df(&message).expect("Failed to get downlink format");
            let result = altitude(&message, df);
            assert_eq!(result, Some(14300));
        }
    }

    #[test]
    fn test_alt_e() {
        if let Some(message) = decoder::get_message("A020100A10020A80F000004F24AF") {
            let df = decoder::df(&message).expect("Failed to get downlink format");
            let result = altitude(&message, df);
            assert_eq!(result, None);
        }
    }
}
