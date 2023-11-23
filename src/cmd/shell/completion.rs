use crate::cmd;
use anyhow::Result;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};

pub const NAME: &str = "completion";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("comp")
        .about("Print shell completion for supported shells")
        .arg_required_else_help(true)
        .arg(
            Arg::new("shell")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    if let Some(shell) = matches.get_one::<Shell>("shell").copied() {
        let mut cmd = cmd::cli();

        print_completions(shell, &mut cmd);
    }

    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
