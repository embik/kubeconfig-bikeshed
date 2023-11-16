use clap::{Arg, Command};

pub mod import;
pub mod prune;
pub mod switch;

pub fn cli() -> Command {
    Command::new("kubeconfig-bike-shed")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(true)
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::SetTrue)
                .global(true)
                .help("Enable verbose (debug) logging"),
        )
        .subcommand(import::command())
        .subcommand(switch::command())
    // TODO: add subcommand 'fetch' to fetch kubeconfigs from remote systems.
    // TODO: add subcommand 'prune' to clean up kubeconfigs for which server URLs no longer
    // respond.
}
