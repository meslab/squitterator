use squitterator::{
    initialize_logger, set_observer_coords_from_str, spawn_reader_thread, Args, Plane,
};

use clap::Parser;
use std::{
    collections::HashMap,
    io,
    sync::{Arc, RwLock},
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
