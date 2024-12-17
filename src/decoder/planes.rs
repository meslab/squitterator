use log::debug;

use crate::AppCounters;
use crate::Args;
use chrono::{DateTime, Utc};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::{
    plane::{format_simple_display, DisplayFlags, Plane},
    UpdateFromDownlink, DF,
};

pub struct Planes {
    pub aircrafts: Arc<RwLock<HashMap<u32, Plane>>>,
}

impl Planes {
    pub fn new() -> Self {
        Planes {
            aircrafts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn update_aircraft(
        &mut self,
        downlink: &DF,
        message: &[u32],
        df: u32,
        icao: u32,
        args: &Args,
    ) {
        if let Ok(mut planes) = self.aircrafts.write() {
            planes
                .entry(icao)
                .and_modify(|p| {
                    if df < 20 && !&args.use_update_method {
                        p.update_from_downlink(downlink)
                    } else {
                        p.update(message, df, args.relaxed)
                    }
                })
                .or_insert(Plane::from_downlink(downlink, icao));
        }
    }

    pub(crate) fn cleanup(&mut self, app_state: &mut AppCounters, now: DateTime<Utc>) {
        if let Ok(mut planes) = self.aircrafts.write() {
            if app_state.cleanup_count > 10 {
                planes.retain(|_, plane| {
                    let elapsed = now.signed_duration_since(plane.timestamp).num_seconds();
                    if elapsed < 60 {
                        true
                    } else {
                        debug!("Plane {} has been removed from view", plane.icao);
                        false
                    }
                });
                planes.shrink_to_fit();
                app_state.reset_cleanup_count();
            }

            app_state.increment_cleanup_count();
        };
    }

    pub fn print(&self, args: &Args, display_flags: &DisplayFlags) {
        let planes = self
            .aircrafts
            .read()
            .expect("Failed to acquire read lock on planes.");
        let mut planes_vector: Vec<(&u32, &Plane)> = planes.iter().collect();
        planes_vector.sort_by_cached_key(|&(k, _)| k);
        for order_by in &args.order_by {
            for c in order_by.chars() {
                match c {
                    'a' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| p.altitude);
                    }
                    'A' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| p.altitude);
                        planes_vector.reverse();
                    }
                    'c' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| p.category);
                    }
                    'C' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| {
                            -(((p.category.0 << 1) | p.category.1) as i32)
                        });
                    }
                    'd' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| {
                            p.distance_from_observer.unwrap_or(0.0) as i32
                        });
                    }
                    'D' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| {
                            p.distance_from_observer.unwrap_or(0.0) as i32
                        });
                        planes_vector.reverse();
                    }
                    'N' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| p.lat as i32);
                    }
                    'S' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| -(p.lat as i32));
                    }
                    'W' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| p.lon as i32);
                    }
                    'E' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| -(p.lon as i32));
                    }
                    's' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| p.squawk);
                    }
                    'V' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| -(p.vrate.unwrap_or(0)));
                    }
                    'v' => {
                        planes_vector.sort_by_cached_key(|&(_, p)| p.vrate.unwrap_or(0));
                    }
                    _ => {}
                }
            }
        }

        print!(
            "{}",
            planes_vector.iter().fold(String::new(), |acc, (_, plane)| {
                acc + &format!("{}\n", format_simple_display(*plane, display_flags))
            })
        );
    }
}

impl Default for Planes {
    fn default() -> Self {
        Self::new()
    }
}
