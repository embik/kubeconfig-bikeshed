use clap::{arg, Command};
use url::Url;

use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

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

pub fn execute(
    config_path: &PathBuf,
    kubeconfig: Option<&PathBuf>,
    name: Option<&String>,
) -> Result<(), Box<dyn Error>> {
    let buf = kubeconfig.ok_or("no kubeconfig path found")?;
    let path = buf.clone().into_os_string();
    let kubeconfig = kubeconfig::get(&path)?;

    // read the name from the command line flag; if it's not set,
    // extract the hostname and use that as name.
    let name: String = match name {
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

    let kubeconfig_path = config_path.join(format!("{}.kubeconfig", name));

    log::debug!("importing kubeconfig to {}", kubeconfig_path.display());

    // TODO: prompt the user for confirmation to override instead of
    // throwing an error.
    if kubeconfig_path.exists() {
        return Err(Box::new(ImportError::FileExists(
            kubeconfig_path.display().to_string(),
        )));
    }

    let file = File::create(&kubeconfig_path)?;
    let file = BufWriter::new(file);
    serde_yaml::to_writer(file, &kubeconfig)?;

    log::info!("imported kubeconfig to {}", kubeconfig_path.display());

    Ok(())
}
