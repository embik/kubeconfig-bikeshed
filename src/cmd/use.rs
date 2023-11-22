use std::path::Path;

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

use crate::config;
use crate::kubeconfig;

pub const NAME: &str = "use";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("u")
        .about("Use a kubeconfig by name and print shell snippet to source")
        .arg(
            Arg::new("kubeconfig")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String)),
        )
        .arg_required_else_help(true)
}

pub fn execute(config_path: &Path, matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let mut requires_store = false;

    let config = match matches
        .get_one::<String>("kubeconfig")
        .ok_or("failed to get kubeconfig argument")
    {
        Ok(s) if s == "-" => {
            let last_active = config::get_last_active(config_path)?;
            log::debug!("found {last_active} as last active kubeconfig");
            last_active
        }
        Ok(s) => {
            requires_store = true;
            s.to_string()
        }
        Err(e) => return Err(e.into()),
    };

    let kubeconfig_path = config_path.join(format!("{config}.kubeconfig"));

    match kubeconfig::get(&kubeconfig_path) {
        Ok(_) => {
            if requires_store {
                config::save_last_active(config_path, &config)?;
                log::debug!("stored {config} as last active kubeconfig");
            }

            print!("export KUBECONFIG={}", kubeconfig_path.display());
            Ok(())
        }
        Err(err) => Err(err),
    }
}
