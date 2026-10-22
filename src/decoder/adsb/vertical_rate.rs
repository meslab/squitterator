pub(crate) fn vertical_rate(message: &[u32]) -> Option<i32> {
    crate::flag_and_range_value(message, 69, 70, 78)
        .filter(|&f| f.1 != 0)
        .map(|(sign, value)| vertical_rate_value(sign, value))
}

fn vertical_rate_value(sign: u32, value: u32) -> i32 {
    let value = ((value - 1) << 6) as i32;
    match sign {
        1 => -value,
        _ => value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_rate_value() {
        assert_eq!(vertical_rate_value(0, 14), 832);
        assert_eq!(vertical_rate_value(1, 14), -832);
    }

    #[test]
    fn test_vertical_rate() {
        // TC19 subtype 1, vertical rate = -832 ft/min (descent).
        let message: Vec<u32> = "8D485020994409940838175B284F"
            .chars()
            .map(|c| c.to_digit(16).unwrap())
            .collect();
        assert_eq!(vertical_rate(&message), Some(-832));
    }

    #[test]
    fn test_vertical_rate_unavailable() {
        // Same frame with the value field (bits 70-78) zeroed means
        // "vertical rate information unavailable" and must not underflow.
        let message: Vec<u32> = "8D485020994409940000175B284F"
            .chars()
            .map(|c| c.to_digit(16).unwrap())
            .collect();
        assert_eq!(vertical_rate(&message), None);
    }
}
