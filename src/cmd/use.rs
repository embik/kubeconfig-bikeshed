use crate::{config, kubeconfig};
use anyhow::{anyhow, Result};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::Path;

pub const NAME: &str = "use";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("u")
        .about("Use a kubeconfig by name and print shell snippet to source")
        .arg(
            Arg::new("kubeconfig")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String)),
        )
        .arg_required_else_help(true)
}

pub fn execute(config_path: &Path, matches: &ArgMatches) -> Result<()> {
    let mut requires_store = false;

    let config = match matches
        .get_one::<String>("kubeconfig")
        .ok_or_else(|| anyhow!("failed to get kubeconfig argument"))
    {
        Ok(s) if s == "-" => {
            let last_active = config::get_last_active(config_path)?;
            log::debug!("found {last_active} as last active kubeconfig");
            last_active
        }
        Ok(s) if s == "[unset]" => {
            log::debug!("unsetting KUBECONFIG environment variable");
            print!("unset KUBECONFIG");
            return Ok(());
        }
        Ok(s) => {
            requires_store = true;
            s.to_string()
        }
        Err(e) => return Err(e),
    };

    let kubeconfig_path = config_path.join(format!("{config}.kubeconfig"));

    if kubeconfig::get(&kubeconfig_path).is_ok() {
        if requires_store {
            config::save_last_active(config_path, &config)?;
            log::debug!("stored {config} as last active kubeconfig");
        }

        print!("export KUBECONFIG={}", kubeconfig_path.display());
        return Ok(());
    }

    Err(anyhow!("Failed to load Kubeconfig!"))
}
