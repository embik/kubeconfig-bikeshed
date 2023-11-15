use clap::{arg, Command};
use url::Url;

use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

mod error;
mod kubeconfig;

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
                .value_parser(clap::value_parser!(OsString)),
        )
        .arg_required_else_help(true)
}

pub fn execute(
    kubeconfig: Option<&PathBuf>,
    name: Option<&OsString>,
) -> Result<(), Box<dyn Error>> {
    let buf = kubeconfig.ok_or("no kubeconfig path found")?;
    let path = buf.clone().into_os_string();
    let kubeconfig = kubeconfig::get(&path)?;

    // read the name from the command line flag; if it's not set,
    // extract the hostname and use that as name.
    let name: OsString = match name {
        Some(str) => str.clone(),
        None => {
            log::debug!("no name passed via flag, reading it from kubeconfig server URL");
            let hostname = kubeconfig::get_hostname(&kubeconfig)?;
            let url = Url::parse(hostname.as_str())?;
            let host = url
                .host_str()
                .ok_or("failed to parse host from server URL")?;
            OsString::from(host)
        }
    };

    log::debug!("using {:?} as name for kubeconfig file and context", name);

    let config_path = get_config_path()?;
    let kubeconfig_path = config_path.join(name);

    let kubeconfig_path_str = kubeconfig_path
        .to_str()
        .ok_or("failed to get string representation of path")?;
    log::debug!("writing kubeconfig to {kubeconfig_path_str}");

    let file = File::create(kubeconfig_path)?;
    let file = BufWriter::new(file);
    serde_yaml::to_writer(file, &kubeconfig)?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let home_path = home::home_dir().ok_or("could not find home directory")?;
    Ok(Path::new(&home_path)
        .join(".config")
        .join("kbs")
        .to_path_buf())
}
