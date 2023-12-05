use crate::config::Output;
use crate::metadata::{self, Metadata};
use anyhow::{anyhow, Result};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::collections::btree_map::BTreeMap;
use std::{fs, path::Path};

pub const NAME: &str = "list";

pub fn command() -> Command {
    Command::new(NAME)
        .visible_alias("ls")
        .about("List available kubeconfigs")
        .arg(
            Arg::new("labels")
                .help("Selector (label query) to filter on. Supports key=value comma-separated values")
                .long("selector")
                .short('l')
                .required(false)
                .num_args(0..)
                .value_delimiter(',')
                .value_parser(metadata::labels::parse_key_val),
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
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .required(false)
                .action(ArgAction::Set)
                .default_value("name")
                .value_parser(value_parser!(Output)),
        )


        .arg_required_else_help(false)
}

pub fn execute(config_dir: &Path, matches: &ArgMatches) -> Result<()> {
    log::debug!("looking for kubeconfigs in {}", config_dir.display());

    let output = matches
        .get_one::<Output>("output")
        .ok_or_else(|| anyhow!("cannot read output"))?;

    let selectors = matches
        .get_many::<(String, String)>("labels")
        .map(|values_ref| values_ref.into_iter().collect::<Vec<&(String, String)>>());

    let metadata_path = metadata::file_path(config_dir);
    log::debug!("loading metadata from {}", metadata_path.display());
    let metadata = match Metadata::from_file(&metadata_path) {
        Ok(metadata) => metadata,
        // TODO: don't ignore failing to parse metadata
        Err(_) => Metadata::new(),
    };

    // print table header
    if *output == Output::Table {
        println!("{0: <25}\t{1: <25}", "NAME", "LABELS");
    }

    let files = fs::read_dir(config_dir)?;
    for file in files {
        let file = file?.path();

        if !is_kubeconfig(&file) {
            continue;
        }

        let name = file
            .file_stem()
            .ok_or_else(|| anyhow!("cannot determine basename"))?
            .to_str()
            .ok_or_else(|| anyhow!("cannot convert file path to string"))?;

        let labels = match metadata.get(name) {
            Some(m) => m.labels.clone().unwrap_or_default(),
            None => BTreeMap::new(),
        };

        if let Some(ref selector) = selectors {
            let mut matched = true;

            for label in selector.iter() {
                let (key, value) = label;
                let opt = labels.get(key);
                matched = opt.is_some() && opt.unwrap() == value;
            }

            if !matched {
                continue;
            }
        }

        log::debug!("found a kubeconfig at {}", file.display());

        println!(
            "{}",
            match *output {
                Output::Name => name.to_string(),
                Output::Table => format!("{0: <25}\t{1: <25}", name, format_labels(&labels)),
            }
        );
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

fn format_labels(map: &BTreeMap<String, String>) -> String {
    map.iter()
        .map(|(key, value)| -> String { format!("{key}={value}") })
        .collect::<Vec<String>>()
        .join(",")
}
