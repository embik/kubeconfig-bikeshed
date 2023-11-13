mod cmd;

use cmd::import;
use env_logger::Builder;
use log::{self, SetLoggerError};
use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;

    match cmd::cli().get_matches().subcommand() {
        Some((import::NAME, sub_matches)) => {
            // run the 'import' subcommand.
            handle(import::execute(
                sub_matches.get_one::<PathBuf>("kubeconfig"),
                sub_matches.get_one::<OsString>("name"),
            ))
        }
        _ => {
            // no subcommand was passed, run fuzzy selection to change KUBECONFIG.
            log::info!("no command");
            Ok(())
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

fn setup_logger() -> Result<(), SetLoggerError> {
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format_target(false)
        .try_init()
}
