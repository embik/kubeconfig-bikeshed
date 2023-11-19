use std::{error::Error, path::Path, process};

use clap::{Arg, ArgMatches, Command};

pub mod import;
pub mod list;
pub mod prune;
pub mod shell;
pub mod r#use;

pub fn cli() -> Command {
    Command::new("kbs")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue)
                .global(true)
                .help("Enable verbose (debug) logging"),
        )
        .subcommands([
            import::command(),
            list::command(),
            r#use::command(),
            shell::command(),
            // TODO: add subcommand 'fetch' to fetch kubeconfigs from remote systems.
            // TODO: add subcommand 'prune' to clean up kubeconfigs with dead server URLs.
        ])
}

pub fn execute(
    config_path: &Path,
    matches: Option<(&str, &ArgMatches)>,
) -> Result<(), Box<dyn Error>> {
    match matches {
        Some((list::NAME, _)) | None => handle(list::execute(config_path)),
        Some((import::NAME, sub_matches)) => handle(import::execute(config_path, sub_matches)),
        Some((r#use::NAME, sub_matches)) => handle(r#use::execute(config_path, sub_matches)),
        Some((shell::NAME, sub_matches)) => handle(shell::execute(sub_matches)),
        _ => {
            log::error!("unknown command");
            process::exit(1);
        }
    }
}

fn handle(res: Result<(), Box<dyn std::error::Error>>) -> Result<(), Box<dyn Error>> {
    match res {
        Err(err) => {
            log::error!("{err}");
            process::exit(1);
        }
        _ => process::exit(0),
    }
}
