mod cmd;
mod config;
mod errors;
mod kubeconfig;

use cmd::{import, switch};
use env_logger::Builder;
use log::{self, SetLoggerError};
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cmd::cli().get_matches();

    setup_logger(matches.get_flag("verbose"))?;

    let config_path = config::get_config_path()?;

    match matches.subcommand() {
        Some((import::NAME, sub_matches)) => {
            // run the 'import' subcommand.
            handle(import::execute(
                &config_path,
                sub_matches.get_one::<PathBuf>("kubeconfig"),
                sub_matches.get_one::<String>("name"),
            ))
        }
        Some((switch::NAME, _)) | None => {
            // no subcommand was passed, run fuzzy selection to change KUBECONFIG.
            handle(switch::execute(&config_path))
        }
        _ => {
            log::error!("unknown command");
            exit(1);
        }
    }
}

fn handle(res: Result<(), Box<dyn std::error::Error>>) -> Result<(), Box<dyn Error>> {
    match res {
        Err(err) => {
            log::error!("{err}");
            exit(1);
        }
        _ => exit(0),
    }
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
