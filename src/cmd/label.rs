use crate::metadata::{self, ConfigMetadata, Metadata};
use anyhow::{anyhow, bail, Result};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::collections::btree_map::BTreeMap;
use std::path::Path;

pub const NAME: &str = "label";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("l")
        .about("Manage labels on kubeconfigs")
        .arg_required_else_help(true)
        .arg(Arg::new("kubeconfig").value_parser(value_parser!(String)))
        .arg(
            Arg::new("labels")
                .value_parser(metadata::labels::parse_key_val::<String, String>)
                .num_args(1..)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("overwrite")
                .help("Allow overwriting existing label values")
                .long("overwrite")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
        )
}

pub fn execute(config_path: &Path, matches: &ArgMatches) -> Result<()> {
    let config = matches
        .get_one::<String>("kubeconfig")
        .ok_or_else(|| anyhow!("failed to get kubeconfig argument"))?;

    let metadata_path = config_path.join(metadata::FILE);
    log::debug!("loading metadata database from {}", metadata_path.display());
    let metadata = match Metadata::from_file(&metadata_path) {
        Ok(metadata) => metadata,
        Err(_) => Metadata::new(),
    };

    let mut labels = BTreeMap::new();

    matches
        .get_many::<(String, String)>("labels")
        .ok_or_else(|| anyhow!("failed to parse labels"))?
        .into_iter()
        .for_each(|(key, value)| {
            labels.insert(key.clone(), value.clone());
        });

    if let Some(config_metadata) = metadata.get(config.to_string()) {
        let mut config_metadata = config_metadata.clone();

        let merged_labels = match &config_metadata.labels {
            Some(existing_labels) => {
                log::debug!("found existing labels for kubeconfig");
                let mut merged_labels = existing_labels.clone();
                for (key, value) in labels.iter() {
                    if let Some(old_value) =
                        merged_labels.insert(key.to_string(), value.to_string())
                    {
                        if !old_value.eq(value) && !matches.get_flag("overwrite") {
                            bail!(
                                "cannot set key '{}' to value '{}', is '{}' already",
                                key,
                                value,
                                old_value
                            );
                        }
                    }
                }

                merged_labels
            }
            None => labels.clone(),
        };

        config_metadata.labels = Some(merged_labels);
        metadata
            .set(config.to_string(), config_metadata)
            .write(&metadata_path)?;

        log::info!("updated labels for {}", config);

        return Ok(());
    }

    metadata
        .set(
            config.to_string(),
            ConfigMetadata {
                labels: Some(labels),
            },
        )
        .write(&metadata_path)?;

    log::info!("updated labels for {}", config);

    Ok(())
}
