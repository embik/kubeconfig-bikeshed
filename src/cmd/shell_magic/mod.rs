use clap::{ArgMatches, Command};

use crate::errors::CmdError;

mod zsh;

pub const NAME: &str = "shell-magic";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("sm")
        .about("Print shell magic for supported shells")
        .arg_required_else_help(true)
        .subcommands([Command::new("zsh").about("Print shell magic for zsh")])
}

pub fn execute(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("zsh", _)) => {
            zsh::print_zsh_magic();
            Ok(())
        }
        _ => Err(Box::new(CmdError::CommandNotFound)),
    }
}
