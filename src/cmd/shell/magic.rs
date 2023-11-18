use clap::builder::PossibleValue;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

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

pub fn execute(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let shell = matches
        .get_one::<Shell>("shell")
        .ok_or("cannot read shell")?;

    match shell {
        Shell::Zsh => {
            let zsh_magic = include_str!("./files/zsh/kbs.source");
            print!("{zsh_magic}");
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
enum Shell {
    Zsh,
}

impl clap::ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[Shell::Zsh]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Shell::Zsh => PossibleValue::new("zsh"),
        })
    }
}
