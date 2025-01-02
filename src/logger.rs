use crate::Result;
use env_logger::{Builder, Env};
use std::{fs::File, io::Write, sync::Mutex};

pub fn initialize_logger(error_log_path: &str) -> Result<()> {
    let file = File::create(error_log_path)?;

    let error_log_file = Mutex::new(file);

    Builder::from_env(Env::default().default_filter_or("error"))
        .format(move |_, record| {
            let mut file = error_log_file
                .lock()
                .expect("Failed to acquire log file lock.");
            writeln!(file, "{} - {}", record.level(), record.args())
        })
        .init();

    Ok(())
}
