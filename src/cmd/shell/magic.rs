use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, value_parser, Arg, ArgAction, ArgMatches, Command};
pub const NAME: &str = "magic";

pub fn command() -> Command {
    Command::new(NAME)
        .about(
            "Print shell magic that overrides the 'kbs' command for supported shells. Requires fzf",
        )
        .arg_required_else_help(true)
        .arg(
            Arg::new("shell")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let shell = matches
        .get_one::<Shell>("shell")
        .ok_or_else(|| anyhow!("cannot read shell"))?;

    let magic = match shell {
        Shell::Zsh => include_str!("./files/zsh/kbs.source"),
        Shell::Bash => include_str!("./files/bash/kbs.source"),
    };
    print!("{magic}");

    Ok(())
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
enum Shell {
    Zsh,
    Bash,
}

impl clap::ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[Shell::Zsh, Shell::Bash]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Shell::Zsh => PossibleValue::new("zsh"),
            Shell::Bash => PossibleValue::new("bash"),
        })
    }
}
