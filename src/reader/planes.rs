use crate::Args;
use squitterator::decoder::header::DisplayFlags;
use squitterator::decoder::{format_simple_display, Plane};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub(super) fn print_planes(
    planes: &Arc<RwLock<HashMap<u32, Plane>>>,
    args: &Args,
    display_flags: &DisplayFlags,
) {
    let planes = planes
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
