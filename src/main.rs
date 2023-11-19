mod cmd;
mod config;
mod errors;
mod kubeconfig;

use env_logger::Builder;
use log::{self, SetLoggerError};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cmd::cli().get_matches();
    let config_path = config::get_config_path()?;

    setup_logger(matches.get_flag("verbose"))?;
    cmd::execute(&config_path, matches.subcommand())
}

fn setup_logger(verbose: bool) -> Result<(), SetLoggerError> {
    let filter_level = match verbose {
        true => log::LevelFilter::Debug,
        false => log::LevelFilter::Info,
    };

    Builder::new()
        .filter_level(filter_level)
        .format_target(false)
        .try_init()
}
