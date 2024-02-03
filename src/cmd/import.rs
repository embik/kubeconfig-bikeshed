use crate::metadata::{self, labels, Metadata};
use crate::{kubeconfig, Error};
use anyhow::{anyhow, bail, Result};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::fs::{self};
use std::path::{Path, PathBuf};

pub const NAME: &str = "import";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("i")
        .about("Import a kubeconfig into data store")
        .arg_required_else_help(true)
        .arg(
            Arg::new("kubeconfig")
                .help("kubeconfig to import. Use a directory to try importing all files in that directory or use '-' to read from stdin")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("name")
                .help("Override context name")
                .long("name")
                .short('n')
                .required(false)
                .num_args(1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("labels")
                .help("List of comma-separated key=value labels to add to the kubeconfig metadata")
                .long("labels")
                .short('l')
                .required(false)
                .value_delimiter(',')
                .value_parser(labels::parse),
        )
        .arg(
            Arg::new("delete")
                .help("Delete original kubeconfig file after import. This flag has no effect when importing a directory")
                .long("delete")
                .short('d')
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
        )
        .arg(
            Arg::new("short")
                .help("Instead of using the FQDN of the server, just use the first part/subdomain")
                .long("short")
                .short('s')
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool))
                .conflicts_with("name"),
        )
        .arg(
            Arg::new("proxy-url")
                .help("Configure a proxy url for the imported kubeconfig")
                .long("proxy-url")
                .short('p')
                .required(false)
                .num_args(1)
                .value_parser(clap::value_parser!(String)),
        )
}

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    let kubeconfig_path = matches
        .get_one::<PathBuf>("kubeconfig")
        .ok_or_else(|| anyhow!("failed to parse kubeconfig argument"))?;

    let labels = labels::from_args(matches, "labels")?;

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

    if kubeconfig_path.is_file() {
        // run import logic.
        let name = kubeconfig::import(
            config_dir,
            kubeconfig_path,
            matches.get_one::<String>("name"),
            matches.get_flag("short"),
            matches.get_one::<String>("proxy-url"),
        )?;

        metadata = metadata.set(
            name,
            metadata::ConfigMetadata {
                labels: Some(labels::to_map(&labels)),
            },
        );

        if matches.get_flag("delete") {
            fs::remove_file(kubeconfig_path)?;
            log::debug!("deleted {}", kubeconfig_path.display());
        }
    } else if kubeconfig_path.is_dir() {
        let files = fs::read_dir(kubeconfig_path)?;
        for file in files {
            let entry = file?;
            let path = entry.path();
            let name = kubeconfig::import(
                config_dir,
                &path,
                None,
                matches.get_flag("short"),
                matches.get_one::<String>("proxy-url"),
            );

            if let Err(err) = name {
                log::warn!(
                    "failed to import {}: {}",
                    path.to_str().unwrap_or("<couldn't unwrap path>"),
                    err
                );
                continue;
            }

            metadata = metadata.set(
                name.unwrap(),
                metadata::ConfigMetadata {
                    labels: Some(labels::to_map(&labels)),
                },
            );
        }
    }

    metadata.write(&metadata_path)?;

    log::debug!(
        "wrote metadata database update to {}",
        metadata_path.display()
    );

    Ok(())
}
