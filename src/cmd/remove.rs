use crate::kubeconfig;
use crate::metadata::{self, Metadata};
use anyhow::Result;
use anyhow::{anyhow, bail};
use clap::{value_parser, Arg, ArgGroup, ArgMatches, Command};
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
                .value_parser(metadata::labels::parse_key_val),
        )
        .group(ArgGroup::new("target")
               .args(["kubeconfig", "selectors"])
               .required(true))
        .arg_required_else_help(true)
}

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    let mut removals: Vec<String> = vec![];

    let metadata_path = metadata::file_path(config_dir);
    log::debug!("loading metadata from {}", metadata_path.display());
    let mut metadata = match Metadata::from_file(&metadata_path) {
        Ok(metadata) => metadata,
        Err(metadata::Error::IO(_, std::io::ErrorKind::NotFound)) => Metadata::new(),
        Err(err) => bail!(err),
    };

    if matches.contains_id("selectors") {
        let selectors = matches
            .get_many::<(String, String)>("selectors")
            .map(|values_ref| values_ref.into_iter().collect::<Vec<&(String, String)>>());

        metadata.kubeconfigs.iter().for_each(|k| {
            if let Some(labels) = &k.1.labels {
                if metadata::labels::matches_labels(&labels, &selectors) {
                    removals.push(k.0.to_string());
                }
            }
        })
    } else {
        let config = matches
            .get_one::<String>("kubeconfig")
            .ok_or_else(|| anyhow!("failed to get kubeconfig argument"))?;

        removals.push(config.to_string());
    }

    for f in removals.iter() {
        let kubeconfig_path = config_dir.join(format!("{f}.kubeconfig"));
        if kubeconfig::get(&kubeconfig_path).is_ok() {
            fs::remove_file(&kubeconfig_path)?;
            log::info!("removed kubeconfig at {}", kubeconfig_path.display());
            metadata = metadata.remove(f);
        } else {
            return Err(anyhow!("Kubeconfig not found: {:?}", f));
        }
    }

    metadata.write(&metadata_path)?;
    log::debug!(
        "wrote metadata database update to {}",
        metadata_path.display()
    );

    Ok(())
}
