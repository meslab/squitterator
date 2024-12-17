mod counters;

pub(crate) use counters::AppCounters;

use crate::{df, icao, message, Args, DisplayFlags, Downlink, Legend, LegendHeaders, Planes, DF};
use log::{debug, error, info, warn};
use std::{
    fs::File,
    io::{BufRead, BufReader, Result, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

fn read_lines<R: BufRead>(reader: R, args: &Args, planes: &mut Planes) -> Result<()> {
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

    let mut app_state = AppCounters::from_update_interval(args.update);

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
                            planes.update_aircraft(&downlink, &message, df, icao, &args);
                            planes.cleanup(&mut app_state, now);
                        }

                        if let Some(ref dlf) = downlink_error_log_file {
                            if let Ok(downlink) = DF::from_message(&message) {
                                let mut dlf =
                                    dlf.lock().expect("Cannot open downlink error log file.");
                                write!(dlf, "{}", downlink)?;
                                debug!("Writing to {:?}", &dlf);
                            }
                        }

                        if !display_flags.quiet() && app_state.is_time_to_refresh(&now, args.update)
                        {
                            clear_screen();

                            headers.print_header();
                            headers.print_separator();

                            planes.print(&args, &display_flags);
                            
                            headers.print_separator();

                            if args.count_df {
                                app_state.print_df_count_line();
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

pub fn spawn_reader_thread(args: Arc<Args>, mut planes: Planes) -> thread::JoinHandle<Result<()>> {
    thread::spawn(move || {
        if !args.tcp.is_empty() {
            connect_and_read_tcp(args, &mut planes)
        } else {
            read_from_file(args, &mut planes)
        }
    })
}

fn connect_and_read_tcp(args: Arc<Args>, planes: &mut Planes) -> Result<()> {
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

fn read_from_file(args: Arc<Args>, planes: &mut Planes) -> Result<()> {
    let file = File::open(&args.source)?;
    let reader = BufReader::new(file);
    read_lines(reader, &args, planes)
}

fn clear_screen() {
    print!("{0}[2J{0}[H{0}[3J", 27 as char);
}
