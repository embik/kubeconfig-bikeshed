use clap::{arg, Command};
use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;

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
    let buf = kubeconfig.ok_or("cannot construct path")?;
    let path = buf.clone().into_os_string();
    let kubeconfig = kubeconfig::get(&path)?;

    let name: OsString = match name {
        Some(str) => str.clone(),
        None => OsString::new(),
    };

    let kubeconfig_str = serde_yaml::to_string(&kubeconfig)?;
    println!("{}", name.to_str().unwrap_or_default());
    println!("{kubeconfig_str}");

    Ok(())
}
