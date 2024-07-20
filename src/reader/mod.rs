mod header;
mod legend;
mod planes;

use header::print_header;
use legend::print_legend;
use planes::print_planes;

use crate::Args;
use squitterator::decoder::{self, df, icao, Downlink};
use squitterator::decoder::{message, Plane};
use std::sync::Arc;
//use squitterator::;
use decoder::UpdateFromDownlink;

use log::{debug, error, warn};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, Result, Write};
use std::sync::Mutex;

pub(super) fn read_lines<R: BufRead>(
    reader: R,
    args: &Args,
    planes: Arc<Mutex<HashMap<u32, Plane>>>,
) -> Result<()> {
    let downlink_error_log_file = args
        .downlink_log
        .as_ref()
        .map(|f| Mutex::new(File::create(f).expect("Unable to create downlink log file")));

    let display_flags = args.display.concat().chars().collect::<Vec<char>>();

    if !display_flags.contains(&'Q') {
        clear_screen();
        print_legend(
            display_flags.contains(&'w'),
            display_flags.contains(&'a'),
            display_flags.contains(&'s'),
            display_flags.contains(&'e'),
        );
    }

    let mut df_count = BTreeMap::new();
    let mut timestamp = chrono::Utc::now() + chrono::Duration::seconds(args.update);
    for line in reader.lines() {
        match line {
            Ok(squitter) => {
                debug!("Squitter: {}", squitter);

                if let Some(message) = message(&squitter) {
                    let df = match df(&message) {
                        Some(df) => df,
                        None => {
                            continue;
                        }
                    };

                    if let Some(m) = &args.log_messages {
                        if m.contains(&df) {
                            error!("DF:{}, L:{}", df, squitter);
                        }
                    }

                    if let Some(only) = &args.filter {
                        if only.iter().all(|&x| x != df) {
                            continue;
                        }
                    }

                    if args.count_df {
                        *df_count.entry(df).or_insert(1) += 1;
                    }

                    if let Some(icao) = icao(&message, df) {
                        if let Ok(downlink) = decoder::DF::from_message(&message) {
                            let mut planes = planes.lock().unwrap();
                            planes
                                .entry(icao)
                                .and_modify(|p| {
                                    if df < 20 && !&args.use_update_method {
                                        p.update_from_downlink(&downlink)
                                    } else {
                                        p.update(&message, df, args.relaxed)
                                    }
                                })
                                .or_insert(Plane::from_downlink(&downlink, icao));
                        }

                        if let Some(ref dlf) = downlink_error_log_file {
                            if let Ok(downlink) = decoder::DF::from_message(&message) {
                                let mut dlf = dlf.lock().unwrap();
                                write!(dlf, "{}", downlink)?;
                                debug!("Writing to {:?}", &dlf);
                            }
                        }

                        let now = chrono::Utc::now();
                        if now.signed_duration_since(timestamp).num_seconds() > args.update {
                            let mut planes = planes.lock().unwrap();
                            planes.retain(|_, plane| {
                                let elapsed =
                                    now.signed_duration_since(plane.timestamp).num_seconds();
                                if elapsed < 60 {
                                    true
                                } else {
                                    debug!("Plane {} has been removed from view", plane.icao);
                                    false
                                }
                            });
                            planes.shrink_to_fit();

                            if !display_flags.contains(&'Q') {
                                clear_screen();
                                print_header(
                                    display_flags.contains(&'w'),
                                    display_flags.contains(&'a'),
                                    display_flags.contains(&'s'),
                                    display_flags.contains(&'A'),
                                    display_flags.contains(&'e'),
                                    true,
                                );
                                print_planes(
                                    &planes,
                                    args,
                                    display_flags.contains(&'w'),
                                    display_flags.contains(&'a'),
                                    display_flags.contains(&'s'),
                                    display_flags.contains(&'A'),
                                    display_flags.contains(&'e'),
                                );
                                print_header(
                                    display_flags.contains(&'w'),
                                    display_flags.contains(&'a'),
                                    display_flags.contains(&'s'),
                                    display_flags.contains(&'A'),
                                    display_flags.contains(&'e'),
                                    false,
                                );

                                if args.count_df {
                                    let result =
                                        df_count.iter().fold(String::new(), |acc, (df, count)| {
                                            acc + &format!("DF{}:{} ", df, count)
                                        });
                                    println!("{}", result);
                                }

                                debug!("{}", planes[&icao]);
                                timestamp = now;
                            }
                        }
                    }
                };
            }
            Err(e) => warn!("Warn: {}", e),
        }
    }
    Ok(())
}

fn clear_screen() {
    print!("{0}[2J{0}[H{0}[3J", 27 as char);
}
