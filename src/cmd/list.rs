use crate::metadata::{self, Metadata};
use anyhow::{anyhow, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::{fs, path::Path};

pub const NAME: &str = "list";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("ls")
        .about("List available kubeconfigs")
        .arg(
            Arg::new("labels")
                .help("Selector (label query) to filter on. Supports key=value comma-separated values")
                .long("selector")
                .short('l')
                .required(false)
                .num_args(0..)
                .value_delimiter(',')
                .value_parser(metadata::labels::parse_key_val::<String, String>),
        )
        .arg(
            Arg::new("unset")
                .help("Show pseudo-element '[unset]'")
                .long("unset")
                .short('u')
                .required(false)
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool)),
        )

        .arg_required_else_help(false)
}

pub fn execute(config_path: &Path, matches: &ArgMatches) -> Result<()> {
    log::debug!("looking for kubeconfigs in {}", config_path.display());

    let selectors = matches
        .get_many::<(String, String)>("labels")
        .map(|values_ref| values_ref.into_iter().collect::<Vec<&(String, String)>>());

    let metadata_path = config_path.join(metadata::FILE);
    log::debug!("loading metadata database from {}", metadata_path.display());
    let metadata = match Metadata::from_file(&metadata_path) {
        Ok(metadata) => metadata,
        Err(_) => Metadata::new(),
    };

    let files = fs::read_dir(config_path)?;
    for file in files {
        let file = file?.path();

        if !is_kubeconfig(&file) {
            continue;
        }

        let file_name = file
            .file_stem()
            .ok_or_else(|| anyhow!("cannot determine basename"))?
            .to_str()
            .ok_or_else(|| anyhow!("cannot convert file name to string"))?;

        if let Some(ref labels) = selectors {
            let mut matched = true;

            if let Some(m) = metadata.get(file_name.to_string()) {
                if let Some(ref config_labels) = m.labels {
                    for label in labels.iter() {
                        let (key, value) = label;
                        let opt = config_labels.get(key);
                        matched = opt.is_some() && opt.unwrap() == value;
                    }
                }
            }

            if !matched {
                continue;
            }
        }

        log::debug!("found {}", file.display());
        println!("{file_name}");
    }

    if matches.get_flag("unset") {
        println!("[unset]");
    }

    Ok(())
}

fn is_kubeconfig(file: &Path) -> bool {
    if !file.is_file() {
        return false;
    }

    matches!(file.extension(), Some(extension) if extension == "kubeconfig")
}
