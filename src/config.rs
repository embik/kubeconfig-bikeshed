use std::error::Error;
use std::path::{Path, PathBuf};

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let home_path = home::home_dir().ok_or("could not find home directory")?;
    Ok(Path::new(&home_path)
        .join(".config")
        .join("kbs")
        .to_path_buf())
}
