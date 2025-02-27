mod arguments;
mod counters;
mod decoder;
mod errors;
mod logger;
mod reader;

pub use arguments::Args;
pub use decoder::{
    DF, DisplayFlags, Downlink, Legend, LegendHeaders, Plane, Planes, UpdateFromDownlink,
    get_downlink_format, get_icao, get_message, set_observer_coords_from_str,
};
pub use errors::AppResult;
pub use logger::initialize_logger;
pub use reader::spawn_reader_thread;
pub(crate) use {
    counters::AppCounters,
    decoder::{flag_and_range_value, range_value},
};
