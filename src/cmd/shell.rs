use anyhow::Result;
use clap::{ArgMatches, Command};

mod completion;
mod magic;

pub const NAME: &str = "shell";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("sh")
        .about("Print various shell related scripts")
        .subcommands([completion::command(), magic::command()])
        .arg_required_else_help(true)
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some((completion::NAME, sub_matches)) => completion::execute(sub_matches),
        Some((magic::NAME, sub_matches)) => magic::execute(sub_matches),
        _ => Ok(()),
    }
}
