mod cmd;

use cmd::import;
use fern::colors::{Color, ColoredLevelConfig};
use log;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;

    match cmd::cli().get_matches().subcommand() {
        Some((import::NAME, sub_matches)) => {
            // run the 'import' subcommand.
            handle(import::execute(
                sub_matches.get_one::<PathBuf>("kubeconfig"),
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

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                colors.color(record.level()),
                humantime::format_rfc3339_seconds(SystemTime::now()),
                message,
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
