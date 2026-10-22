pub(crate) fn ground_movement(message: &[u32]) -> Option<f64> {
    if let Some(value) = crate::range_value(message, 38, 44) {
        let value = value as f64;
        match value {
            0.0 => None,
            1.0 => Some(0.0),
            2.0..=8.0 => Some((value - 1.0) * 0.125),
            9.0..=12.0 => Some(1.0 + (value - 9.0) * 0.25),
            13.0..=38.0 => Some(2.0 + (value - 13.0) * 0.5),
            39.0..=93.0 => Some(15.0 + (value - 39.0)),
            94.0..=108.0 => Some(70.0 + (value - 94.0) * 2.0),
            109.0..=123.0 => Some(100.0 + (value - 109.0) * 5.0),
            124.0 => Some(175.0),
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_movement() {
        let message: Vec<u32> = vec![0; 28];
        assert!(ground_movement(&message).is_none());
    }
}
