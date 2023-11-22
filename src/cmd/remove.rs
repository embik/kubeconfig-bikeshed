use std::{fs, path::Path};

use clap::{value_parser, Arg, ArgMatches, Command};

use crate::kubeconfig;

pub const NAME: &str = "remove";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("rm")
        .about("Delete kubeconfig by name")
        .arg(Arg::new("kubeconfig").value_parser(value_parser!(String)))
        .arg_required_else_help(true)
}

pub fn execute(config_path: &Path, matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = matches
        .get_one::<String>("kubeconfig")
        .ok_or("failed to get kubeconfig argument")?;

    let kubeconfig_path = config_path.join(format!("{config}.kubeconfig"));

    match kubeconfig::get(&kubeconfig_path) {
        Ok(_) => {
            fs::remove_file(&kubeconfig_path)?;
            log::info!("removed kubeconfig at {}", kubeconfig_path.display());
            Ok(())
        }
        Err(err) => Err(err),
    }
}
