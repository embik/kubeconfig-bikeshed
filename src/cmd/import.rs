use clap::{arg, ArgMatches, Command};
use url::Url;

use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use crate::errors::ImportError;
use crate::kubeconfig;

pub const NAME: &str = "import";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("i")
        .about("Import a kubeconfig into store")
        .arg(
            arg!(<KUBECONFIG> "kubeconfig to import")
                .id("kubeconfig")
                .value_parser(clap::value_parser!(std::path::PathBuf)),
        )
        .arg(
            arg!(<NAME> "Override context name")
                .id("name")
                .long("name")
                .short('n')
                .required(false)
                .num_args(1)
                .value_parser(clap::value_parser!(String)),
        )
        .arg_required_else_help(true)
}

pub fn execute(config_path: &Path, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let kubeconfig_path = matches
        .get_one::<PathBuf>("kubeconfig")
        .ok_or("failed to parse kubeconfig argument")?;
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
                .ok_or("failed to parse host from server URL")?;
            host.to_string()
        }
    };

    log::debug!("using {:?} as name for kubeconfig file and context", name);

    let target_path = config_path.join(format!("{}.kubeconfig", name));

    log::debug!("importing kubeconfig to {}", target_path.display());

    // TODO: prompt the user for confirmation to override instead of
    // throwing an error.
    if target_path.exists() {
        return Err(Box::new(ImportError::FileExists(
            kubeconfig_path.display().to_string(),
        )));
    }

    let kubeconfig = kubeconfig::rename_context(&kubeconfig, &name)?;

    let file = File::create(&target_path)?;
    let file = BufWriter::new(file);
    serde_yaml::to_writer(file, &kubeconfig)?;

    log::info!("imported kubeconfig to {}", target_path.display());

    Ok(())
}
