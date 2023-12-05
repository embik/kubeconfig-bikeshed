use crate::metadata::{self, labels, ConfigMetadata, Metadata};
use anyhow::{anyhow, bail, Result};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
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
                .value_parser(metadata::labels::parse_key_val)
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

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    let config = matches
        .get_one::<String>("kubeconfig")
        .ok_or_else(|| anyhow!("failed to get kubeconfig argument"))?;

    let metadata_path = metadata::file_path(config_dir);
    log::debug!("loading metadata from {}", metadata_path.display());
    let metadata = match Metadata::from_file(&metadata_path) {
        Ok(metadata) => metadata,
        Err(metadata::Error::IO(_, std::io::ErrorKind::NotFound)) => {
            log::debug!("failed to find metadata file, creating empty metadata store");
            Metadata::new()
        }
        Err(err) => bail!(err),
    };

    // collect labels from argument into map
    let labels = labels::collect_from_args(matches, "labels")?;

    // if kubeconfig has metadata already, we need to merge labels
    if let Some(config_metadata) = metadata.get(config) {
        let mut config_metadata = config_metadata.clone();

        config_metadata.labels = Some(labels::merge_labels(
            &config_metadata,
            &labels,
            matches.get_flag("overwrite"),
        )?);

        metadata
            .set(config.to_string(), config_metadata)
            .write(&metadata_path)?;

        log::info!("updated labels for {}", config);

        return Ok(());
    }

    // no previous metadata exists for this kubeconfig, so we can safely set it
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
