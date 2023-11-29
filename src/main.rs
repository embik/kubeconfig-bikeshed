use anyhow::Result;

mod cmd;
mod config;
mod kubeconfig;
mod metadata;

fn main() -> Result<()> {
    let matches = cmd::cli().get_matches();
    let config_dir = config::get_config_dir()?;
    setup_logger(matches.get_flag("verbose"))?;

    log::debug!("using {} as configuration directory", config_dir.display());

    if config_dir.is_file() {
        log::error!(
            "configuration directory {} cannot be a file",
            config_dir.display()
        );
        std::process::exit(1);
    }

    if !config_dir.is_dir() {
        log::debug!("creating configuration directory as it does not exist");
        std::fs::create_dir_all(&config_dir).map_err(|err: std::io::Error| -> std::io::Error {
            log::error!("failed to create directory: {err}");
            std::process::exit(1);
        })?;
    }

    cmd::execute(&config_dir, matches.subcommand())
}

fn setup_logger(verbose: bool) -> Result<()> {
    let filter_level = match verbose {
        true => log::LevelFilter::Debug,
        false => log::LevelFilter::Info,
    };

    Ok(env_logger::Builder::new()
        .filter_level(filter_level)
        .format_target(false)
        .try_init()?)
}
