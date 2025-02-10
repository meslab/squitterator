use crate::decoder::{flag_and_range_value, range_value};

pub(crate) fn ground_track(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 45, 46, 52)
        .filter(|&f| f.0 == 1)
        .map(|v| (v.1 * 360) >> 7)
}

pub(crate) fn track_and_groundspeed(
    message: &[u32],
    is_supersonic: bool,
) -> (Option<u32>, Option<u32>) {
    let sp_west = match flag_and_range_value(message, 46, 47, 56) {
        Some((dir_west, speed_west)) => match dir_west {
            1 => -(speed_west as f64 - 1.0),
            _ => speed_west as f64 - 1.0,
        },
        _ => 0.0,
    };

    let sp_south = match flag_and_range_value(message, 57, 58, 67) {
        Some((dir_south, speed_south)) => match dir_south & 1 {
            1 => -(speed_south as f64 - 1.0),
            _ => speed_south as f64 - 1.0,
        },
        _ => 0.0,
    };

    let supersonic = |x| if is_supersonic { x * 4 } else { x };
    let groundspeed = supersonic((sp_west.powi(2) + sp_south.powi(2)).sqrt().floor() as u32);
    let track = (((sp_west as f64).atan2(sp_south).to_degrees().floor() + 360.0) % 360.0) as u32;
    (Some(track), Some(groundspeed))
}

pub(crate) fn heading(message: &[u32]) -> Option<u32> {
    range_value(message, 47, 56)
}

#[cfg(test)]
mod tests {
    use crate::decoder;

    use super::*;

    #[test]
    fn test_track_and_groundspeed() {
        if let Some(message) = decoder::get_message("8DC06A75990D0628B0040C8AA788") {
            if let (Some(track), Some(groundspeed)) = track_and_groundspeed(&message, false) {
                assert_eq!(groundspeed, 416);
                assert_eq!(track, 321);
            };
        };
    }
}
