mod planes;

use chrono::{DateTime, Utc};
pub use planes::print_planes;

use crate::{
    df, icao, message, Args, DisplayFlags, Downlink, Legend, LegendHeaders, Plane,
    UpdateFromDownlink, DF,
};
use log::{debug, error, info, warn};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader, Result, Write},
    net::TcpStream,
    sync::{Arc, Mutex, RwLock},
    thread::{self, sleep},
    time::Duration,
};

struct AppCounters {
    df_count: BTreeMap<u32, i32>,
    timestamp: DateTime<Utc>,
    cleanup_count: u32,
}

impl AppCounters {
    fn from_update(update: i64) -> Self {
        AppCounters {
            df_count: BTreeMap::new(),
            timestamp: chrono::Utc::now() + chrono::Duration::seconds(update),
            cleanup_count: 0u32,
        }
    }

    fn update_count(&mut self, df: u32) {
        *self.df_count.entry(df).or_insert(1) += 1;
    }

    fn reset_cleanup_count(&mut self) {
        self.cleanup_count = 0;
    }

    fn increment_cleanup_count(&mut self) {
        self.cleanup_count += 1;
    }

    fn reset_timestamp(&mut self, now: DateTime<Utc>) {
        self.timestamp = now
    }
}

fn read_lines<R: BufRead>(
    reader: R,
    args: &Args,
    planes: &Arc<RwLock<HashMap<u32, Plane>>>,
) -> Result<()> {
    let downlink_error_log_file = args
        .downlink_log
        .as_ref()
        .map(|f| Mutex::new(File::create(f).expect("Unable to create downlink log file")));

    let display_flags = DisplayFlags::from_arg_str(&args.display.concat());

    if !display_flags.quiet() {
        clear_screen();

        let legend = Legend::from_display_flags(&display_flags);
        legend.print_legend();
    }

    let headers = LegendHeaders::from_display_flags(&display_flags);

    let mut app_state = AppCounters::from_update(args.update);

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
                        app_state.update_count(df);
                    }

                    if let Some(icao) = icao(&message, df) {
                        let now = chrono::Utc::now();
                        if let Ok(downlink) = DF::from_message(&message) {
                            if let Ok(mut planes) = planes.write() {
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

                                if app_state.cleanup_count > 10 {
                                    planes.retain(|_, plane| {
                                        let elapsed = now
                                            .signed_duration_since(plane.timestamp)
                                            .num_seconds();
                                        if elapsed < 60 {
                                            true
                                        } else {
                                            debug!(
                                                "Plane {} has been removed from view",
                                                plane.icao
                                            );
                                            false
                                        }
                                    });
                                    planes.shrink_to_fit();
                                    app_state.reset_cleanup_count();
                                }

                                app_state.increment_cleanup_count();
                            };
                        }

                        if let Some(ref dlf) = downlink_error_log_file {
                            if let Ok(downlink) = DF::from_message(&message) {
                                let mut dlf =
                                    dlf.lock().expect("Cannot open downlink error log file.");
                                write!(dlf, "{}", downlink)?;
                                debug!("Writing to {:?}", &dlf);
                            }
                        }

                        if now.signed_duration_since(app_state.timestamp).num_seconds()
                            > args.update
                            && !display_flags.quiet()
                        {
                            clear_screen();

                            headers.print_header();
                            headers.print_separator();
                            print_planes(planes, args, &display_flags);
                            headers.print_separator();

                            if args.count_df {
                                let result = app_state
                                    .df_count
                                    .iter()
                                    .fold(String::new(), |acc, (df, count)| {
                                        acc + &format!("DF{}:{} ", df, count)
                                    });
                                println!("{}", result);
                            }

                            app_state.reset_timestamp(now);
                        }
                    }
                };
            }
            Err(e) => warn!("Warn: {}", e),
        }
    }
    Ok(())
}

pub fn spawn_reader_thread(
    args: Arc<Args>,
    planes: Arc<RwLock<HashMap<u32, Plane>>>,
) -> thread::JoinHandle<Result<()>> {
    thread::spawn(move || {
        if !args.tcp.is_empty() {
            connect_and_read_tcp(args, &planes)
        } else {
            read_from_file(args, &planes)
        }
    })
}

fn connect_and_read_tcp(args: Arc<Args>, planes: &Arc<RwLock<HashMap<u32, Plane>>>) -> Result<()> {
    loop {
        match TcpStream::connect(&args.tcp) {
            Ok(stream) => {
                info!("Successfully connected to the server {}", &args.tcp);
                let reader = BufReader::new(stream);
                if let Err(e) = read_lines(reader, &args, planes) {
                    error!("Error during reading: {}", e);
                    sleep(Duration::from_secs(5));
                }
            }
            Err(e) => {
                error!("Failed to connect to {}: {}", &args.tcp, e);
                sleep(Duration::from_secs(5));
            }
        }
    }
}

fn read_from_file(args: Arc<Args>, planes: &Arc<RwLock<HashMap<u32, Plane>>>) -> Result<()> {
    let file = File::open(&args.source)?;
    let reader = BufReader::new(file);
    read_lines(reader, &args, planes)
}

fn clear_screen() {
    print!("{0}[2J{0}[H{0}[3J", 27 as char);
}
