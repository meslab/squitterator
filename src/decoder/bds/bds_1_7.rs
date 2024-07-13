use crate::decoder;

#[derive(Debug)]
pub struct Capability {
    pub flags: u32,
    pub bds20: bool,
    pub bds40: bool,
    pub bds44: bool,
    pub bds50: bool,
    pub bds60: bool,
}

impl Capability {
    pub fn new() -> Self {
        Capability {
            flags: 0,
            bds20: false,
            bds40: false,
            bds44: false,
            bds50: false,
            bds60: false,
        }
    }

    pub fn from_data(
        flags: u32,
        bds20: bool,
        bds40: bool,
        bds44: bool,
        bds50: bool,
        bds60: bool,
    ) -> Self {
        Capability {
            flags,
            bds20,
            bds40,
            bds44,
            bds50,
            bds60,
        }
    }
}

impl Default for Capability {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_bds_1_7(message: &[u32]) -> Option<Capability> {
    if let Some((bds20, reserved)) = decoder::flag_and_range_value(message, 39, 61, 88) {
        if bds20 == 1 && reserved == 0 {
            decoder::range_value(message, 33, 56).map(|capability| {
                Capability::from_data(
                    capability,
                    (bds20 & 1) == 1,
                    ((capability >> 15) & 1) == 1,
                    ((capability >> 11) & 1) == 1,
                    ((capability >> 8) & 1) == 1,
                    (capability & 1) == 1,
                )
            })
        } else {
            None
        }
    } else {
        None
    }
}
