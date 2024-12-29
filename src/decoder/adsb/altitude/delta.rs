pub(crate) fn altitude_delta(message: &[u32]) -> Option<i32> {
    crate::flag_and_range_value(message, 81, 82, 88)
        .filter(|&f| f.1 != 0)
        .map(|(sign, value)| delta(sign, value))
}

fn delta(sign: u32, value: u32) -> i32 {
    match sign {
        1 => -(value as i32) * 25,
        _ => value as i32 * 25,
    }
}
