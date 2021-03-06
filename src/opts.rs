use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "my-iot", author, about)]
pub struct Opts {
    /// Show only warnings and errors
    #[structopt(short = "s", long = "silent", conflicts_with = "verbose")]
    pub silent: bool,

    /// Show all log messages
    #[structopt(short = "v", long = "verbose", conflicts_with = "silent")]
    pub verbose: bool,

    /// Suppress timestamps in logs, useful with journald
    #[structopt(long = "suppress-log-timestamps")]
    pub suppress_log_timestamps: bool,

    /// Setting files
    #[structopt(parse(from_os_str), env = "MYIOT_SETTINGS", default_value = "my-iot.toml")]
    pub settings: Vec<PathBuf>,

    /// Prints version information
    #[structopt(short = "V", long = "version")]
    pub version: bool,
}
