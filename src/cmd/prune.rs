use crate::kubeconfig;
use crate::metadata::{self, Metadata};
use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};
use futures::executor;
use std::fs;
use std::path::Path;

pub const NAME: &str = "prune";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("p")
        .about("Remove kubeconfigs for Kubernetes API servers that are no longer accessible")
        .arg(
            Arg::new("dry-run")
                .help("Only list kubeconfigs with unreachable Kubernetes API servers, do not delete them")
                .long("dry-run")
                .short('n')
                .required(false)
                .action(ArgAction::Set)
                .default_value("true")
                .default_missing_value("true")
                .num_args(0..=1)
                .require_equals(true)
                .value_parser(clap::value_parser!(bool)),
        )
        .arg(
            Arg::new("selectors")
                .help("Selector (label query) to filter on. Supports key=value comma-separated values")
                .long("selector")
                .short('l')
                .required(false)
                .num_args(0..)
                .value_delimiter(',')
                .value_parser(metadata::selectors::parse),
        )
}

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    log::debug!("looking for kubeconfigs in {}", config_dir.display());

    let selectors = metadata::selectors::from_args(matches, "selectors")?;
    let dry_run = matches.get_flag("dry-run");

    let metadata_path = metadata::file_path(config_dir);
    log::debug!("loading metadata from {}", metadata_path.display());
    let mut metadata = Metadata::from_file(&metadata_path)?;

    let kubeconfigs = kubeconfig::list(config_dir, &metadata, Some(selectors))?;

    for (name, _) in kubeconfigs.iter() {
        log::info!("checking if '{name}' should be pruned");
        let (kubecfg_path, kubecfg) = kubeconfig::get(&config_dir, name)?;

        let options = kube::config::KubeConfigOptions {
            cluster: None,
            context: None,
            user: None,
        };

        let config = executor::block_on(kube::Config::from_custom_kubeconfig(kubecfg, &options))?;
        let client = kube::client::Client::try_from(config)?;

        let response = executor::block_on(client.apiserver_version());

        if response.is_err() {
            if !dry_run {
                fs::remove_file(&kubecfg_path)?;
                log::info!("pruned kubeconfig '{name}' at {}", kubecfg_path.display());
                metadata = metadata.remove(name);
            } else {
                log::info!("'{name}' should be pruned");
            }
        };
    }

    Ok(())
}
