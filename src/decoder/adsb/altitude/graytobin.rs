use crate::decoder;

pub(super) fn graytobin(message: &[u32]) -> (u32, u32) {
    if let Some(code) = decoder::ma_code(message) {
        let n = (((code >> 4) & 1) << 10)
            | (((code >> 2) & 1) << 9)
            | (((code >> 12) & 1) << 8)
            | (((code >> 10) & 1) << 7)
            | (((code >> 8) & 1) << 6)
            | (((code >> 7) & 1) << 5)
            | (((code >> 5) & 1) << 4)
            | (((code >> 3) & 1) << 3)
            | (((code >> 13) & 1) << 2)
            | (((code >> 11) & 1) << 1)
            | ((code >> 13) & 1);
        let mut mask = 0x80;
        let mut cp = false;
        let mut result = 0;
        for _ in 1..=16 {
            if (n & mask) != 0 {
                cp = !cp;
            }
            if cp {
                result |= mask;
            }
            mask >>= 1;
        }

        let sub = n & 7;
        let high = result >> 3;
        let low = match high & 1 {
            0 => match sub {
                4 => 4,
                6 => 3,
                3 => 1,
                2 => 2,
                _ => 0,
            },
            _ => match sub {
                1 => 4,
                3 => 3,
                6 => 1,
                2 => 2,
                _ => 0,
            },
        };
        (high as u32, low as u32)
    } else {
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graytobin() {
        if let Some(message) = decoder::get_message("A8281200200464B3CF7820CD194C") {
            let (high, low) = graytobin(&message);
            assert_eq!(high, 31);
            assert_eq!(low, 0);
        }
    }

    #[test]
    fn test_graytobin_e() {
        if let Some(message) = decoder::get_message("A020100A10020A80F000004F24AF") {
            let (high, low) = graytobin(&message);
            assert_eq!(high, 2);
            assert_eq!(low, 0);
        }
    }
}
