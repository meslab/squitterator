mod reader;
use reader::read_lines;
use squitterator::decoder::{self, Plane};

use crate::decoder::Coordinates;
use clap::Parser;
use env_logger::{Builder, Env};
use log::{error, info};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, sleep};
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(
    version = "v0.3.0",
    author = "Anton Sidorov tonysidrock@gmail.com",
    about = "ADS-B squitter decoder"
)]

struct Args {
    #[clap(short, long, help = "Count squitters by type")]
    count_df: bool,

    #[clap(
        short,
        long,
        default_value = "aAews",
        help = "Display plane patameters\na - angles, A - altitude, s - speed\ne - extra info, w - weather\nQ - quiet"
    )]
    display: Vec<String>,

    #[clap(short = 'D', long, default_value = None)]
    downlink_log: Option<String>,

    #[clap(short = 'l', long, default_value = "sq.errors.log")]
    error_log: String,

    #[clap(short, long, default_value = None, help = "Process only specific DF messages\n -f 21 -f 4 - DF4 and DF21,\n -f 21 - only DF21, etc")]
    filter: Option<Vec<u32>>,

    #[clap(short='F', long, default_value = None)]
    format: Option<String>,

    #[clap(short='M', long, default_value = None)]
    log_messages: Option<Vec<u32>>,

    #[clap(
        short,
        long,
        default_value = "sA",
        help = "s - squawk, a,A - altitude,\nc,C - category, N, S, E, W - location,\nv,V - vertical rate"
    )]
    order_by: Vec<String>,

    #[clap(
        short = 'O',
        long,
        default_value = "52.66411442720024, -8.622299905360963"
    )]
    observer_coord: Option<String>,

    #[clap(short = 'R', long, help = "Relaxed Capabilities check EHS")]
    relaxed: bool,

    #[clap(
        short,
        long,
        conflicts_with = "tcp",
        default_value = "rec/squitters.txt"
    )]
    source: String,

    #[clap(
        short,
        long,
        conflicts_with = "source",
        required = false,
        default_value = ""
    )]
    tcp: String,

    #[clap(short, long, default_value = "3")]
    update: i64,

    #[clap(short = 'U', long, help = "Use Plain::update() exclusively")]
    use_update_method: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let error_log_file = File::create(&args.error_log).expect("Unable to create log file");
    let error_log_file = Mutex::new(error_log_file);

    let planes: Arc<RwLock<HashMap<u32, Plane>>> = Arc::new(RwLock::new(HashMap::new()));

    let coords = if let Some(coord_str) = &args.observer_coord {
        match coord_str.parse::<Coordinates>() {
            Ok(coords) => Some((coords.lat, coords.lon)),
            Err(e) => {
                eprintln!("Error parsing coordinates: {}", e);
                None
            }
        }
    } else {
        None
    };

    decoder::set_observer_coords(coords);

    // Initialize the logger
    Builder::from_env(Env::default().default_filter_or("error"))
        .format(move |_, record| {
            let mut error_log_file = error_log_file.lock().unwrap();
            writeln!(error_log_file, "{} - {}", record.level(), record.args())
        })
        .init();

    let reader_thread = {
        let planes = planes.clone();
        thread::spawn(move || match !args.tcp.is_empty() {
            true => loop {
                let stream = match TcpStream::connect(&args.tcp) {
                    Ok(stream) => {
                        info!("Successfully connected to the server {}", &args.tcp);
                        stream
                    }
                    Err(e) => {
                        error!("Failed to connect: {}", e);
                        continue;
                    }
                };
                let reader = BufReader::new(stream);
                if let Err(e) = read_lines(reader, &args, planes.clone()) {
                    error!("Error during reading: {}", e);
                    sleep(Duration::from_secs(5));
                    continue;
                }
            },
            _ => {
                let file = File::open(&args.source)?;
                let reader = BufReader::new(file);
                read_lines(reader, &args, planes.clone())
            }
        })
    };

    reader_thread.join().unwrap()
}
