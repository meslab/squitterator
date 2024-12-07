use crate::decoder;

pub(crate) fn is_bds_4_5(message: &[u32]) -> Option<f64> {
    if decoder::goodflags(message, 33, 34, 35)
        && decoder::goodflags(message, 36, 37, 38)
        && decoder::goodflags(message, 39, 40, 41)
        && decoder::goodflags(message, 42, 43, 44)
        && decoder::goodflags(message, 45, 46, 47)
        && decoder::goodflags(message, 48, 49, 58)
        && decoder::goodflags(message, 59, 60, 60)
        && decoder::goodflags(message, 71, 72, 83)
        && !decoder::goodflags(message, 33, 84, 88)
    {
        decoder::temperature_4_5(message).filter(|&temp| temp <= 45.0)
    } else {
        None
    }
}
