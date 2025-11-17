use squitterator::{
    initialize_logger, set_observer_coords_from_str, spawn_reader_thread, AppResult, Args, Planes,
};

use clap::Parser;
use std::sync::Arc;

fn main() -> AppResult<()> {
    let args = Arc::new(Args::parse());

    if let Some(error_log_file) = &args.error_log {
        initialize_logger(error_log_file)?;
    };

    if let Some(coord_str) = &args.observer_coord {
        set_observer_coords_from_str(coord_str)
    };

    let planes = Planes::new();

    let reader_thread = spawn_reader_thread(args, planes);
    reader_thread
        .join()
        .expect("Couldn't join on the associated thread")?;
    Ok(())
}
