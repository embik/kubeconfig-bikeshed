use crate::kubeconfig;
use anyhow::anyhow;
use anyhow::Result;
use clap::{value_parser, Arg, ArgMatches, Command};
use std::{fs, path::Path};

pub const NAME: &str = "remove";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("rm")
        .about("Delete kubeconfig by name")
        .arg(Arg::new("kubeconfig").value_parser(value_parser!(String)))
        .arg_required_else_help(true)
}

pub fn execute(config_path: &Path, matches: &ArgMatches) -> Result<()> {
    let config = matches
        .get_one::<String>("kubeconfig")
        .ok_or_else(|| anyhow!("failed to get kubeconfig argument"))?;

    let kubeconfig_path = config_path.join(format!("{config}.kubeconfig"));

    if kubeconfig::get(&kubeconfig_path).is_ok() {
        fs::remove_file(&kubeconfig_path)?;
        log::info!("removed kubeconfig at {}", kubeconfig_path.display());
        return Ok(());
    }
    Err(anyhow!("Kubeconfig not found: {:?}", kubeconfig_path))
}
