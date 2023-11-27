use anyhow::{anyhow, Result};
use clap::builder::PossibleValue;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

const ACTIVE_FILE_NAME: &str = "active";

pub fn get_config_path() -> Result<PathBuf> {
    // attempt to respect XDG_CONFIG_HOME, fall back to $HOME/.config if it's not
    // set.
    let base = match env::var("XDG_CONFIG_HOME") {
        Ok(s) => s.into(),
        Err(_) => home::home_dir()
            .ok_or_else(|| anyhow!("could not determine home directory"))?
            .join(".config"),
    };

    Ok(base.join("kbs"))
}

pub fn get_last_active(config_path: &Path) -> io::Result<String> {
    fs::read_to_string(config_path.join(ACTIVE_FILE_NAME))
}

pub fn save_last_active(config_path: &Path, name: &String) -> io::Result<()> {
    fs::write(config_path.join(ACTIVE_FILE_NAME), name)
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum Output {
    Name,
    Table,
}

impl clap::ValueEnum for Output {
    fn value_variants<'a>() -> &'a [Self] {
        &[Output::Name, Output::Table]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Output::Name => PossibleValue::new("name"),
            Output::Table => PossibleValue::new("table"),
        })
    }
}

impl From<Output> for clap::builder::OsStr {
    fn from(value: Output) -> clap::builder::OsStr {
        value.into()
    }
}
