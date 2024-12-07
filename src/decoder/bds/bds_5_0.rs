use super::goodflags;
use crate::decoder::*;

pub struct TrackAndTurn {
    pub roll_angle: Option<i32>,
    pub track_angle: Option<u32>,
    pub track_angle_rate: Option<i32>,
    pub ground_speed: Option<u32>,
    pub true_airspeed: Option<u32>,
}

impl TrackAndTurn {
    pub fn new() -> Self {
        TrackAndTurn {
            roll_angle: None,
            track_angle: None,
            track_angle_rate: None,
            ground_speed: None,
            true_airspeed: None,
        }
    }

    pub fn from_data(
        roll_angle: Option<i32>,
        track_angle: Option<u32>,
        track_angle_rate: Option<i32>,
        ground_speed: Option<u32>,
        true_airspeed: Option<u32>,
    ) -> Self {
        TrackAndTurn {
            roll_angle,
            track_angle,
            track_angle_rate,
            ground_speed,
            true_airspeed,
        }
    }
}

impl Default for TrackAndTurn {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_bds_5_0(message: &[u32]) -> Option<TrackAndTurn> {
    if !goodflags(message, 33, 34, 43)
        || !goodflags(message, 44, 45, 55)
        || !goodflags(message, 56, 57, 66)
        || !goodflags(message, 67, 68, 77)
        || !goodflags(message, 78, 79, 88)
    {
        return None;
    }

    let track = TrackAndTurn::from_data(
        roll_angle_5_0(message).filter(|x| (-50..=50).contains(x)),
        track_angle_5_0(message).filter(|x| (0..=360).contains(x)),
        track_angle_rate_5_0(message).filter(|x| (0..=2046).contains(x)),
        ground_speed_5_0(message).filter(|x| (0..=600).contains(x)),
        true_airspeed_5_0(message).filter(|x| (0..=500).contains(x)),
    );

    if track.ground_speed.is_some()
        && track.true_airspeed.is_some()
        && track.roll_angle.is_some()
        && track.track_angle.is_some()
        && track.track_angle_rate.is_some()
        && track.ground_speed?.abs_diff(track.true_airspeed?) < 200
    {
        Some(track)
    } else {
        None
    }
}
