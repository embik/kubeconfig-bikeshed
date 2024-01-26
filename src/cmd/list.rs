use crate::config::Output;
use crate::kubeconfig;
use crate::metadata::{self, Metadata};
use anyhow::{anyhow, Result};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::collections::btree_map::BTreeMap;
use std::path::Path;

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
    let metadata = Metadata::from_file(&metadata_path)?;

    let kubeconfigs = kubeconfig::list(config_dir, &metadata, selectors.clone())?;

    // print table header
    if *output == Output::Table {
        println!("{0: <25}\t{1: <25}", "NAME", "LABELS");
    }

    // loop over all kubeconfigs we found
    for (name, labels) in kubeconfigs {
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

fn format_labels(map: &Option<BTreeMap<String, String>>) -> String {
    if let Some(labels) = map {
        return labels
            .iter()
            .map(|(key, value)| -> String { format!("{key}={value}") })
            .collect::<Vec<String>>()
            .join(",");
    }

    "".to_string()
}
