use crate::metadata::{self, Metadata};
use crate::Error;
use crate::{config, kubeconfig};
use anyhow::Result;
use anyhow::{anyhow, bail};
use clap::{value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use std::{fs, path::Path};

pub const NAME: &str = "remove";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("rm")
        .visible_alias("delete")
        .about("Remove kubeconfig from data store")
        .arg(Arg::new("kubeconfig").value_parser(value_parser!(String)))
        .arg(
            Arg::new("selectors")
                .help("Selector (label query) to filter on. Supports key=value comma-separated values")
                .long("selector")
                .short('l')
                .num_args(0..)
                .value_delimiter(',')
                .value_parser(metadata::selectors::parse),
        )
        .arg(
            Arg::new("active")
                .help("Remove the currently active kubeconfig")
                .long("active")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),

            )
        .group(ArgGroup::new("target")
               .args(["kubeconfig", "selectors", "active"])
               .required(true))
        .arg_required_else_help(true)
}

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    let metadata_path = metadata::file_path(config_dir);
    log::debug!("loading metadata from {}", metadata_path.display());
    let mut metadata = match Metadata::from_file(&metadata_path) {
        Ok(metadata) => metadata,
        Err(Error::IO(err)) if err.kind() == std::io::ErrorKind::NotFound => Metadata::new(),
        Err(err) => bail!(err),
    };

    let removals = match matches.contains_id("selectors") {
        true => {
            let selectors = metadata::selectors::from_args(matches, "selectors")?;

            kubeconfig::list(config_dir, &metadata, Some(selectors))?
        }
        false => {
            if matches.contains_id("kubeconfig") {
                let config = matches
                    .get_one::<String>("kubeconfig")
                    .ok_or_else(|| anyhow!("failed to get kubeconfig argument"))?;

                vec![
                    (kubeconfig::ListEntry {
                        name: config.to_string(),
                        labels: None,
                    }),
                ]
            } else if matches.contains_id("active") {
                let current = config::get_last_active(config_dir)?;
                vec![
                    (kubeconfig::ListEntry {
                        name: current,
                        labels: None,
                    }),
                ]
            } else {
                vec![]
            }
        }
    };

    for entry in removals.iter() {
        let kubeconfig_path = kubeconfig::get_path(config_dir, &entry.name);
        if kubeconfig::get(config_dir, &entry.name).is_ok() {
            fs::remove_file(&kubeconfig_path)?;
            log::info!("removed kubeconfig at {}", kubeconfig_path.display());
            metadata = metadata.remove(&entry.name);
        } else {
            return Err(anyhow!("kubeconfig not found: {:?}", &entry.name));
        }
    }

    metadata.write(&metadata_path)?;
    log::debug!(
        "wrote metadata database update to {}",
        metadata_path.display()
    );

    Ok(())
}
