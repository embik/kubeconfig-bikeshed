use std::path::PathBuf;

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

use crate::kubeconfig;

pub const NAME: &str = "path";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("p")
        .about("Print full path to a specific kubeconfig in kbs store")
        .arg(
            Arg::new("kubeconfig")
                .action(ArgAction::Set)
                .value_parser(value_parser!(String)),
        )
        .arg_required_else_help(true)
}

pub fn execute(
    config_path: &PathBuf,
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let kube_config = matches
        .get_one::<String>("kubeconfig")
        .ok_or("failed to get kubeconfig argument")?;

    let kube_config_path = config_path.join(format!("{kube_config}.kubeconfig"));

    match kubeconfig::get(&kube_config_path) {
        Ok(_) => {
            print!("{}", kube_config_path.display());
            Ok(())
        }
        Err(err) => Err(err),
    }
}
