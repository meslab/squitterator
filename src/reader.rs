use crate::Args;
use log::{debug, warn};
use squitterator::adsb::message;
use squitterator::adsb::{clean_squitter, df, icao};
use squitterator::plane::{format_simple_display, Plane};
use squitterator::process::generate_ais;
use squitterator::process::icao_decode;
use squitterator::process::squitter_decode;
use std::collections::HashMap;
use std::io::{BufRead, Result};

pub fn read_lines<R: BufRead>(
    reader: R,
    args: &Args,
    planes: &mut HashMap<u32, Plane>,
) -> Result<()> {
    let mut timestamp = chrono::Utc::now();
    for line in reader.lines() {
        match line {
            Ok(squitter) => {
                debug!("Squitter: {}", squitter);
                if let Some(message) = message(&squitter) {
                    let df = df(&message);
                    if let Some(only) = args.only {
                        if only != df {
                            continue;
                        }
                    }

                    let now = chrono::Utc::now();
                    if args.ais {
                        generate_ais(&message, &squitter);
                    }

                    if args.decode {
                        squitter_decode(&message, df);
                    }

                    if args.icao {
                        icao_decode(&message, df, clean_squitter(&squitter).unwrap().as_str());
                    }
                    if args.planes {
                        if let Some(icao) = icao(&message, df) {
                            planes
                                .entry(icao)
                                .and_modify(|p| p.update(&message, df))
                                .or_insert(Plane::from_message(&message, df, icao));

                            if now.signed_duration_since(timestamp).num_seconds() > args.refresh {
                                clear_screen();
                                print_header(args.wide);
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
                                for (_, plane) in planes.iter() {
                                    println!("{}", format_simple_display(plane, args.wide));
                                }
                                debug!("Squirter: {}", squitter);
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
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);
}

fn print_header(wide: bool) {
    let headers = [
        ("ICAO", 6),
        ("RG", 2),
        ("ALT", 5),
        ("SQWK", 4),
        ("CALLSIGN", 8),
        ("LATITUDE", 9),
        ("LONGITUDE", 11),
        ("GSPD", 4),
        ("TRK", 3),
        ("VRATE", 5),
    ];

    let extra_headers = [("DF", 2), ("TC", 2), ("V", 1), ("S", 1), ("LPC", 3)];

    for &(header, width) in &headers {
        print!("{:width$} ", header, width = width);
    }
    if wide {
        for &(header, width) in &extra_headers {
            print!("{:width$} ", header, width = width);
        }
    }
    println!("LC");

    for &(_, width) in &headers {
        print!("{:-<width$} ", "", width = width);
    }
    if wide {
        for &(_, width) in &extra_headers {
            print!("{:-<width$} ", "", width = width);
        }
    }
    println!("--");
}
