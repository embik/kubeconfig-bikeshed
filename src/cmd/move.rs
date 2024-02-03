use crate::metadata::{self, Metadata};
use crate::{kubeconfig, Error};
use anyhow::{anyhow, bail, Result};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::Path;

pub const NAME: &str = "move";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("mv")
        .about("Rename a kubeconfig in store")
        .arg_required_else_help(true)
        .arg(
            Arg::new("name")
                .help("kubeconfig to rename")
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("destination")
                .help("target name for the kubeconfig")
                .value_parser(value_parser!(String)),
        )
}

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    let source = matches
        .get_one::<String>("name")
        .ok_or_else(|| anyhow!("failed to parse name argument"))?;

    let destination = matches
        .get_one::<String>("destination")
        .ok_or_else(|| anyhow!("failed to parse destination argument"))?;

    let metadata_path = metadata::file_path(config_dir);
    log::debug!("loading metadata from {}", metadata_path.display());
    let metadata = match Metadata::from_file(&metadata_path) {
        Ok(metadata) => metadata,
        Err(Error::IO(err)) if err.kind() == std::io::ErrorKind::NotFound => {
            log::debug!("failed to find metadata file, creating empty metadata store");
            Metadata::new()
        }
        Err(err) => bail!(err),
    };

    kubeconfig::get(config_dir, source)?;

    if kubeconfig::get(config_dir, destination).is_ok() {
        bail!("{destination} already exists");
    }

    kubeconfig::r#move(config_dir, source, destination)?;

    metadata
        .rename(source, destination)?
        .write(&metadata_path)?;

    log::debug!(
        "wrote metadata database update to {}",
        metadata_path.display()
    );

    Ok(())
}
