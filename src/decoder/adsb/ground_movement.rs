pub(crate) fn ground_movement(message: &[u32]) -> Option<f64> {
    if let Some(value) = crate::range_value(message, 38, 44) {
        let value = value as f64;
        match value {
            1.0 => Some(0.0),
            2.0..=8.0 => Some(value / 8.0),
            9.0..=12.0 => Some(value / 4.0),
            13.0..=38.0 => Some(value / 2.0),
            39.0..=93.0 => Some(value),
            94.0..=108.0 => Some(value * 2.0),
            109.0..=123.0 => Some(value * 5.0),
            124.0 => Some(175.0),
            _ => None,
        }
    } else {
        None
    }
}
