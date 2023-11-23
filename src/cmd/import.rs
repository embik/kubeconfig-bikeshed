use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use url::Url;

use std::fs::{self};

use anyhow::{anyhow, bail, Result};

use crate::errors::ImportError;
use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use crate::kubeconfig;

pub const NAME: &str = "import";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("i")
        .about("Import a kubeconfig into store")
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
            Arg::new("delete")
                .help("Delete original kubeconfig file after import")
                .long("delete")
                .short('d')
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
        )
        .arg_required_else_help(true)
}

pub fn execute(config_path: &Path, matches: &ArgMatches) -> Result<()> {
    let kubeconfig_path = matches
        .get_one::<PathBuf>("kubeconfig")
        .ok_or_else(|| anyhow!("failed to parse kubeconfig argument"))?;
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
            host.to_string()
        }
    };

    log::debug!("using {} as name for kubeconfig file and context", name);

    let target_path = config_path.join(format!("{}.kubeconfig", name));

    log::debug!("importing kubeconfig to {}", target_path.display());

    // TODO: prompt the user for confirmation to override instead of
    // throwing an error.
    if target_path.exists() {
        bail!(ImportError::FileExists(format!("{:?}", target_path)));
    }

    let kubeconfig = kubeconfig::rename_context(&kubeconfig, &name)?;

    let file = File::create(&target_path)?;
    let file = BufWriter::new(file);
    serde_yaml::to_writer(file, &kubeconfig)?;

    log::info!("imported kubeconfig to {}", target_path.display());

    if matches.get_flag("delete") {
        fs::remove_file(kubeconfig_path)?;
        log::debug!("deleted {}", kubeconfig_path.display());
    }

    Ok(())
}
