use clap::Command;
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const NAME: &str = "version";

pub fn command() -> Command {
    Command::new(NAME).alias("v").about("Print version")
}

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    println!("{VERSION}");
    Ok(())
}
