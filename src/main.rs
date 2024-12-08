mod reader;
use reader::spawn_reader_thread;
use squitterator::decoder::{set_observer_coords_from_str, Plane};

use clap::Parser;
use env_logger::{Builder, Env};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(
    version = "v0.3.1",
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

    #[clap(short = 'l', long, default_value = "/dev/null")]
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

    if let Some(coord_str) = &args.observer_coord {
        set_observer_coords_from_str(coord_str)
    };

    initialize_logger(&args.error_log);

    let planes: Arc<RwLock<HashMap<u32, Plane>>> = Arc::new(RwLock::new(HashMap::new()));

    let reader_thread = spawn_reader_thread(Arc::new(args), planes);
    reader_thread
        .join()
        .expect("Couldn't join on the associated thread")
}

fn initialize_logger(error_log_path: &str) {
    if error_log_path == "/dev/null" {
        return;
    }

    let file = match File::create(error_log_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create log file '{}': {}", error_log_path, e);
            return;
        }
    };

    let error_log_file = Mutex::new(file);

    Builder::from_env(Env::default().default_filter_or("error"))
        .format(move |_, record| {
            let mut file = error_log_file
                .lock()
                .expect("Failed to acquire log file lock.");
            writeln!(file, "{} - {}", record.level(), record.args())
        })
        .init();
}
