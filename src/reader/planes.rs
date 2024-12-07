use crate::Args;
use squitterator::decoder::{format_simple_display, Plane};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub(super) struct DisplayFlags {
    pub(super) bits: u8,
}

impl DisplayFlags {
    pub(super) fn new(
        weather: bool,
        angles: bool,
        speed: bool,
        altitude: bool,
        extra: bool,
    ) -> Self {
        let mut bits = 0u8;
        if weather {
            bits |= 1 << 0;
        }
        if angles {
            bits |= 1 << 1;
        }
        if speed {
            bits |= 1 << 2;
        }
        if altitude {
            bits |= 1 << 3;
        }
        if extra {
            bits |= 1 << 4;
        }
        DisplayFlags { bits }
    }

    pub(super) fn weather(&self) -> bool {
        self.bits & (1 << 0) != 0
    }
    pub(super) fn angles(&self) -> bool {
        self.bits & (1 << 1) != 0
    }
    pub(super) fn speed(&self) -> bool {
        self.bits & (1 << 2) != 0
    }
    pub(super) fn altitude(&self) -> bool {
        self.bits & (1 << 3) != 0
    }
    pub(super) fn extra(&self) -> bool {
        self.bits & (1 << 4) != 0
    }
}

pub(super) fn print_planes(
    planes: &Arc<RwLock<HashMap<u32, Plane>>>,
    args: &Args,
    print_flags: &DisplayFlags,
) {
    let planes = planes.read().unwrap();
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
            acc + &format!(
                "{}\n",
                format_simple_display(
                    *plane,
                    print_flags.weather(),
                    print_flags.angles(),
                    print_flags.speed(),
                    print_flags.altitude(),
                    print_flags.extra()
                )
            )
        })
    )
}
