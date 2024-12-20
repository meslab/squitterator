use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(
    version = "v0.3.4",
    author = "Anton Sidorov tonysidrock@gmail.com",
    about = "ADS-B squitter decoder"
)]
pub struct Args {
    #[clap(short, long, help = "Count squitters by type")]
    pub count_df: bool,

    #[clap(
        short,
        long,
        default_value = "aAews",
        help = "Display plane patameters\na - angles, A - altitude, s - speed\ne - extra info, w - weather\nQ - quiet"
    )]
    pub display: Vec<String>,

    #[clap(short = 'D', long, default_value = None)]
    pub downlink_log: Option<String>,

    #[clap(short = 'l', long, default_value = None)]
    pub error_log: Option<String>,

    #[clap(short, long, default_value = None, help = "Process only specific DF messages\n -f 21 -f 4 - DF4 and DF21,\n -f 21 - only DF21, etc")]
    pub filter: Option<Vec<u32>>,

    #[clap(short='F', long, default_value = None)]
    pub format: Option<String>,

    #[clap(short='M', long, default_value = None)]
    pub log_messages: Option<Vec<u32>>,

    #[clap(
        short,
        long,
        default_value = "sA",
        help = "s - squawk, a,A - altitude,\nc,C - category, N, S, E, W - location,\nv,V - vertical rate"
    )]
    pub order_by: Vec<String>,

    #[clap(
        short = 'O',
        long,
        default_value = "52.66411442720024, -8.622299905360963"
    )]
    pub observer_coord: Option<String>,

    #[clap(short = 'R', long, help = "Relaxed Capabilities check EHS")]
    pub relaxed: bool,

    #[clap(
        short,
        long,
        conflicts_with = "tcp",
        default_value = "rec/squitters.txt"
    )]
    pub source: String,

    #[clap(
        short,
        long,
        conflicts_with = "source",
        required = false,
        default_value = ""
    )]
    pub tcp: String,

    #[clap(short, long, default_value = "3")]
    pub update: i64,

    #[clap(short = 'U', long, help = "Use Plain::update() exclusively")]
    pub use_update_method: bool,
}
