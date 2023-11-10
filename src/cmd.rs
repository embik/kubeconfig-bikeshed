use clap::{Arg, Command};

pub mod import;

pub fn cli() -> Command {
    Command::new("kbs")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(true)
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .num_args(0)
                .global(true)
                .help("Show debug information"),
        )
        .subcommand(import::command())
}
