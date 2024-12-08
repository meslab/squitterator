mod reader;
use reader::spawn_reader_thread;

use squitterator::{
    arguments::Args,
    decoder::{set_observer_coords_from_str, Plane},
};

use clap::Parser;
use env_logger::{Builder, Env};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    sync::{Arc, Mutex, RwLock},
};

fn main() -> io::Result<()> {
    let args = Args::parse();

    initialize_logger(&args.error_log);

    if let Some(coord_str) = &args.observer_coord {
        set_observer_coords_from_str(coord_str)
    };

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
