use crate::metadata::{self, labels, ConfigMetadata, Metadata};
use crate::Error;
use anyhow::{anyhow, bail, Result};
use clap::{value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use std::path::Path;

pub const NAME: &str = "label";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("l")
        .about("Manage labels on kubeconfigs in the data store")
        .arg_required_else_help(true)
        .arg(
            Arg::new("labels")
                .value_parser(metadata::labels::parse_key_val)
                .value_delimiter(',')
                .required(true)
        )
        .arg(Arg::new("kubeconfig")
             .long("name")
             .short('n')
             .value_parser(value_parser!(String))
             .required_unless_present("selectors")
        )
        .arg(
            Arg::new("selectors")
                .help("Selector (label query) to filter on. Supports key=value comma-separated values")
                .long("selector")
                .short('l')
                .value_delimiter(',')
                .value_parser(metadata::labels::parse_key_val)
                .conflicts_with("kubeconfig"),
        )
        .arg(
            Arg::new("overwrite")
                .help("Allow overwriting existing label values")
                .long("overwrite")
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
        )
        .group(ArgGroup::new("target").args(&["selectors", "kubeconfig"]).required(true))
        .arg_required_else_help(true)
}

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    let mut to_label: Vec<String> = vec![];

    let metadata_path = metadata::file_path(config_dir);
    log::debug!("loading metadata from {}", metadata_path.display());
    let mut metadata = match Metadata::from_file(&metadata_path) {
        Ok(metadata) => metadata,
        Err(Error::IO(err)) if err.kind() == std::io::ErrorKind::NotFound => {
            log::debug!("failed to find metadata file, creating empty metadata store");
            Metadata::new()
        }
        Err(err) => bail!(err),
    };

    if matches.contains_id("selectors") && !matches.contains_id("kubeconfig") {
        let selectors = matches
            .get_many::<(String, String)>("selectors")
            .map(|values_ref| values_ref.into_iter().collect::<Vec<&(String, String)>>());

        metadata.kubeconfigs.iter().for_each(|k| {
            if let Some(labels) = &k.1.labels {
                if metadata::labels::matches_labels(&labels, &selectors) {
                    to_label.push(k.0.to_string());
                }
            }
        })
    } else if matches.contains_id("kubeconfig") && !matches.contains_id("selectors") {
        let config = matches
            .get_one::<String>("kubeconfig")
            .ok_or_else(|| anyhow!("failed to get kubeconfig argument"))?;

        to_label.push(config.to_string());
    } else {
        bail!("cannot set both name and label selector");
    }

    // collect labels from argument into map
    let labels = labels::collect_from_args(matches, "labels")?;

    for f in to_label.iter() {
        // if kubeconfig has metadata already, we need to merge labels
        if let Some(config_metadata) = metadata.get(f) {
            let mut config_metadata = config_metadata.clone();

            config_metadata.labels = Some(labels::merge_labels(
                &config_metadata,
                &labels,
                matches.get_flag("overwrite"),
            )?);

            metadata
                .set(f.to_string(), config_metadata)
                .write(&metadata_path)?;

            log::info!("updated labels for {}", f);

            return Ok(());
        }

        // no previous metadata exists for this kubeconfig, so we can safely set it
        metadata = metadata.set(
            f.to_string(),
            ConfigMetadata {
                labels: Some(labels.clone()),
            },
        );

        log::info!("updated labels for {}", f);
    }

    metadata.write(&metadata_path)?;
    log::debug!(
        "wrote metadata database update to {}",
        metadata_path.display()
    );

    Ok(())
}
