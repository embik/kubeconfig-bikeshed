use anyhow::Result;
use clap::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const NAME: &str = "version";

pub fn command() -> Command {
    Command::new(NAME).visible_alias("v").about("Print version")
}

pub fn execute() -> Result<()> {
    println!("v{VERSION}");
    Ok(())
}
