use crate::kubeconfig;
use crate::metadata::{self, Metadata};
use anyhow::Result;
use anyhow::{anyhow, bail};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::{fs, path::Path};

pub const NAME: &str = "remove";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("rm")
        .visible_alias("delete")
        .about("Remove kubeconfig from data store")
        .arg(Arg::new("kubeconfig").value_parser(value_parser!(String)))
        .arg_required_else_help(true)
}

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    let config = matches
        .get_one::<String>("kubeconfig")
        .ok_or_else(|| anyhow!("failed to get kubeconfig argument"))?;

    let kubeconfig_path = config_dir.join(format!("{config}.kubeconfig"));

    let metadata_path = metadata::file_path(config_dir);
    log::debug!("loading metadata from {}", metadata_path.display());
    let metadata = match Metadata::from_file(&metadata_path) {
        Ok(metadata) => metadata,
        Err(metadata::Error::IO(_, std::io::ErrorKind::NotFound)) => Metadata::new(),
        Err(err) => bail!(err),
    };

    if kubeconfig::get(&kubeconfig_path).is_ok() {
        fs::remove_file(&kubeconfig_path)?;
        log::info!("removed kubeconfig at {}", kubeconfig_path.display());

        metadata.remove(config).write(&metadata_path)?;
        log::debug!(
            "wrote metadata database update to {}",
            metadata_path.display()
        );

        return Ok(());
    }

    Err(anyhow!("Kubeconfig not found: {:?}", kubeconfig_path))
}
