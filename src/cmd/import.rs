use clap::{arg, Arg, Command};
use std::error::Error;
use std::path::PathBuf;

mod config;

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
            Arg::new("name")
                .long("name")
                .short('n')
                .num_args(1)
                .help("Override context name"),
        )
        .arg_required_else_help(true)
}

pub fn execute(kubeconfig: Option<&PathBuf>) -> Result<(), Box<dyn Error>> {
    let kubeconfig_path = match kubeconfig {
        Some(path_buf) => path_buf,
        None => panic!("cannot construct path"),
    };
    let path_os_string = kubeconfig_path.clone().into_os_string();
    let kubeconfig = config::get_kubeconfig(&path_os_string)?;

    println!("{:?}", kubeconfig);

    Ok(())
}
