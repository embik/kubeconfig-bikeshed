use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use std::path::Path;

pub mod import;
pub mod list;
pub mod prune;
pub mod remove;
pub mod shell;
pub mod r#use;
pub mod version;

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
            remove::command(),
            version::command(),
            // TODO: add subcommand 'fetch' to fetch kubeconfigs from remote systems.
            // TODO: add subcommand 'prune' to clean up kubeconfigs with dead server URLs.
        ])
}

pub fn execute(config_path: &Path, matches: Option<(&str, &ArgMatches)>) -> Result<()> {
    match matches {
        Some((list::NAME, _)) | None => handle(list::execute(config_path)),
        Some((import::NAME, sub_matches)) => handle(import::execute(config_path, sub_matches)),
        Some((r#use::NAME, sub_matches)) => handle(r#use::execute(config_path, sub_matches)),
        Some((shell::NAME, sub_matches)) => handle(shell::execute(sub_matches)),
        Some((remove::NAME, sub_matches)) => handle(remove::execute(config_path, sub_matches)),
        Some((version::NAME, _)) => handle(version::execute()),
        _ => {
            log::error!("unknown command");
            std::process::exit(1);
        }
    }
}

fn handle(res: Result<()>) -> Result<()> {
    match res {
        Err(err) => {
            log::error!("{err}");
            std::process::exit(1);
        }
        _ => std::process::exit(0),
    }
}
