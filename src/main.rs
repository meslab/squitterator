use squitterator::{
    initialize_logger, set_observer_coords_from_str, spawn_reader_thread, Args, Planes, Result,
};

use clap::Parser;
use std::sync::Arc;

fn main() -> Result<()> {
    let args = Args::parse();

    initialize_logger(&args.error_log);

    if let Some(coord_str) = &args.observer_coord {
        set_observer_coords_from_str(coord_str)
    };

    let planes = Planes::new();

    let reader_thread = spawn_reader_thread(Arc::new(args), planes);
    Ok(reader_thread
        .join()
        .expect("Couldn't join on the associated thread")?)
}
