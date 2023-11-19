use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    // attempt to respect XDG_CONFIG_HOME, fall back to $HOME/.config if it's not set.
    let path = match env::var("XDG_CONFIG_HOME") {
        Ok(s) => Path::new(&s).to_path_buf(),
        Err(_) => {
            let path = home::home_dir().ok_or("could not determine home directory")?;
            path.join(".config")
        }
    };

    Ok(path.join("kbs"))
}
