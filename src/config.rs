use anyhow::{anyhow, Result};
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
        Err(_) => home::home_dir().ok_or_else(|| anyhow!("could not determine home directory"))?,
    };

    Ok(base.join("kbs"))
}

pub fn get_last_active(config_path: &Path) -> io::Result<String> {
    fs::read_to_string(config_path.join(ACTIVE_FILE_NAME))
}

pub fn save_last_active(config_path: &Path, name: &String) -> io::Result<()> {
    fs::write(config_path.join(ACTIVE_FILE_NAME), name)
}
