pub(crate) fn vertical_rate(message: &[u32]) -> Option<i32> {
    crate::flag_and_range_value(message, 69, 70, 78)
        .filter(|&f| f.1 != 1)
        .map(|(sign, value)| vertical_rate_value(sign, value))
}

fn vertical_rate_value(sign: u32, value: u32) -> i32 {
    let value = ((value - 1) << 6) as i32;
    match sign {
        1 => -value,
        _ => value,
    }
}
