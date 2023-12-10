use crate::kubeconfig;
use crate::metadata::{self, labels, Metadata};
use anyhow::{anyhow, bail, Result};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::fs::{self};
use std::os::unix::fs::PermissionsExt;
use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};
use url::Url;

pub const NAME: &str = "import";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("i")
        .about("Import a kubeconfig into data store")
        .arg_required_else_help(true)
        .arg(
            Arg::new("kubeconfig")
                .help("kubeconfig to import")
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
                .long("set-labels")
                .short('l')
                .required(false)
                .value_delimiter(',')
                .value_parser(metadata::labels::parse_key_val),
        )
        .arg(
            Arg::new("delete")
                .help("Delete original kubeconfig file after import")
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
}

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    let kubeconfig_path = matches
        .get_one::<PathBuf>("kubeconfig")
        .ok_or_else(|| anyhow!("failed to parse kubeconfig argument"))?;

    // collect labels from argument into map
    let labels = match matches.contains_id("labels") {
        true => Some(labels::collect_from_args(matches, "labels")?),
        false => None,
    };

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

    let kubeconfig = kubeconfig::get(kubeconfig_path)?;

    // read the name from the command line flag; if it's not set,
    // extract the hostname and use that as name.
    let name: String = match matches.get_one::<String>("name") {
        Some(str) => str.clone(),
        None => {
            log::debug!("no name passed via flag, reading it from kubeconfig server URL");
            let hostname = kubeconfig::get_hostname(&kubeconfig)?;
            let url = Url::parse(hostname.as_str())?;
            let host = url
                .host_str()
                .ok_or_else(|| anyhow!("failed to parse host from server URL"))?;

            match matches.get_flag("short") {
                true => host.split_once('.').unwrap_or((&host, "")).0.to_string(),
                false => host.to_string(),
            }
        }
    };

    log::debug!("using {} as name for kubeconfig file and context", name);

    let target_path = config_dir.join(format!("{}.kubeconfig", name));

    // TODO: prompt the user for confirmation to override instead of
    // throwing an error.
    if target_path.exists() {
        bail!(
            "kubeconfig {} already exists at {}",
            name,
            target_path.display()
        );
    }

    let kubeconfig = kubeconfig::rename_context(&kubeconfig, &name)?;

    let file = File::create(&target_path)?;
    file.set_permissions(fs::Permissions::from_mode(0o600))?;
    let file = BufWriter::new(file);
    serde_yaml::to_writer(file, &kubeconfig)?;

    log::info!("imported kubeconfig to {}", target_path.display());

    metadata
        .set(name, metadata::ConfigMetadata { labels })
        .write(&metadata_path)?;

    log::debug!(
        "wrote metadata database update to {}",
        metadata_path.display()
    );

    if matches.get_flag("delete") {
        fs::remove_file(kubeconfig_path)?;
        log::debug!("deleted {}", kubeconfig_path.display());
    }

    Ok(())
}
