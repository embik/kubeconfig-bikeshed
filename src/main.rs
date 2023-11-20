mod cmd;
mod config;
mod errors;
mod kubeconfig;

use env_logger::Builder;
use log::{self, SetLoggerError};
use std::error::Error;
use std::fs;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cmd::cli().get_matches();
    let config_path = config::get_config_path()?;

    setup_logger(matches.get_flag("verbose"))?;
    log::debug!("using {} as configuration directory", config_path.display());

    if !config_path.is_dir() {
        log::debug!("creating configuration directory as it does not exist");
        fs::create_dir_all(&config_path).or_else(
            |err: std::io::Error| -> Result<(), Box<dyn Error>> {
                log::error!("failed to create directory: {err}");
                process::exit(1);
            },
        )?;
    }

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
