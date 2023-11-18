use clap::{Arg, Command};

pub mod import;
pub mod list;
pub mod path;
pub mod prune;
pub mod shell;

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
            path::command(),
            shell::command(),
            // TODO: add subcommand 'fetch' to fetch kubeconfigs from remote systems.
            // TODO: add subcommand 'prune' to clean up kubeconfigs with dead server URLs.
        ])
}
