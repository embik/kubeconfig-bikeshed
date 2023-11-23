use anyhow::Result;
use clap::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const NAME: &str = "version";

pub fn command() -> Command {
    Command::new(NAME).alias("v").about("Print version")
}

pub fn execute() -> Result<()> {
    println!("{VERSION}");
    Ok(())
}
