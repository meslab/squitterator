use squitterator::{
    AppResult, Args, Planes, initialize_logger, set_observer_coords_from_str, spawn_reader_thread,
};

use clap::Parser;
use log::error;
use std::sync::Arc;

fn main() -> AppResult<()> {
    let args = Arc::new(Args::parse());
    ctrlc::set_handler(move || {
        print!("\x1B[0m\x1B[?25h");
        print!("\x1b[?1049l");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    std::panic::set_hook(Box::new(|info| {
        // Reset terminal before printing the panic message
        print!("\x1b[?1049l");
        eprintln!("Panic occurred: {:?}", info);
    }));
    print!("\x1b[?1049h");

    // Initialize error logging if specified
    if let Some(error_log_file) = &args.error_log {
        initialize_logger(error_log_file)?;
    }

    // Set observer coordinates if provided
    if let Some(coord_str) = &args.observer_coord {
        set_observer_coords_from_str(coord_str);
    }

    let planes = Planes::new();
    let reader_thread = spawn_reader_thread(args, planes);

    // Wait for the reader thread to complete
    reader_thread
        .join()
        .map_err(|_| std::io::Error::other("Failed to join reader thread: thread panicked"))?
        .map_err(|e| {
            error!("Reader thread error: {}", e);
            e
        })?;

    Ok(())
}
