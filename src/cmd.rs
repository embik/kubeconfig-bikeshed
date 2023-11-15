use clap::{Arg, Command};

pub mod import;

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
}
