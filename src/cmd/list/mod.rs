use std::fs;
use std::path::PathBuf;

use clap::Command;

pub const NAME: &str = "list";

pub fn command() -> Command {
    Command::new(NAME)
        .alias("ls")
        .about("List available kubeconfig")
        .arg_required_else_help(false)
}

pub fn execute(config_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("looking for kubeconfigs in {}", config_path.display());

    let files = fs::read_dir(config_path)?;
    for file in files {
        let file = file?.path();

        if !is_kubeconfig(&file) {
            continue;
        }

        let file_name = file.file_stem().ok_or("cannot determine basename")?;

        log::debug!("found {}", file.display());
        println!(
            "{}",
            file_name
                .to_str()
                .ok_or("cannot convert file name to string")?
        );
    }

    Ok(())
}

fn is_kubeconfig(file: &PathBuf) -> bool {
    if !file.is_file() {
        return false;
    }

    match file.extension() {
        Some(extension) if extension == "kubeconfig" => true,
        _ => false,
    }
}
