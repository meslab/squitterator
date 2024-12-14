use crate::decoder::{flag_and_range_value, status_flag_and_range_value};

pub(crate) fn temperature_4_4(message: &[u32]) -> Option<f64> {
    flag_and_range_value(message, 56, 57, 66).map(|(sign, value)| temp_4_4(sign, value))
}

fn temp_4_4(sign: u32, value: u32) -> f64 {
    let temp = value as i32;
    match sign {
        0 => temp as f64 * 0.25,
        _ => (!temp + 1) as f64 * 0.25,
    }
}

fn wind_speed(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 37, 38, 46)
        .filter(|&(status, _)| status == 1)
        .map(|(_, speed)| speed)
}

fn wind_direction(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 37, 47, 55)
        .filter(|&(status, _)| status == 1)
        .map(|(_, value)| (value * 180) >> 8)
}

pub(crate) fn wind_4_4(message: &[u32]) -> Option<(u32, u32)> {
    wind_speed(message).map(|wind_speed| {
        wind_direction(message).map(|wind_direction| (wind_speed, wind_direction))
    })?
}

pub(crate) fn turbulence_4_4(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 79, 80, 81)
        .filter(|&(status, _)| status == 1)
        .map(|(_, value)| value)
}

pub(crate) fn humidity_4_4(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 82, 83, 88)
        .filter(|&(status, _)| status == 1)
        .map(|(_, value)| (value * 100) >> 6)
}

pub(crate) fn pressure_4_4(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 67, 68, 78)
        .filter(|&(status, _)| status == 1)
        .map(|(_, value)| value)
}

pub(crate) fn temperature_4_5(message: &[u32]) -> Option<f64> {
    status_flag_and_range_value(message, 48, 49, 50, 58)
        .filter(|&(status, _, _)| status == 1)
        .map(|(_, sign, value)| temp_4_5(sign, value))
}

fn temp_4_5(sign: u32, value: u32) -> f64 {
    match sign {
        1 => -(value as f64 * 0.25),
        _ => value as f64 * 0.25,
    }
}

#[cfg(test)]
mod tests {
    use super::temp_4_4;

    #[test]
    fn test_power() {
        assert_eq!(2_i32.pow(9), 1 << 9);
        for x in 0..=539 {
            assert_eq!(-x, !x + 1);
        }
        assert_eq!(2_i32.pow(9), 1 << 9);
    }

    #[test]
    fn test_humidity_range() {
        assert_eq!((0b111111 * 100) >> 6, 98);
        assert_eq!(100 >> 6, 1);
        assert_eq!((0b000000 * 100) >> 6, 0);
    }

    #[test]
    fn test_temp_4_4() {
        assert_eq!(temp_4_4(1, 0b1111111111), -255.75);
        assert_eq!(temp_4_4(0, 0b1111111111), 255.75);
    }
}
