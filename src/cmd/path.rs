use std::path::PathBuf;

use clap::Command;

pub const NAME: &str = "path";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("p")
        .about("Print config directory base path")
        .arg_required_else_help(false)
}

pub fn execute(config_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", config_path.display());
    Ok(())
}
