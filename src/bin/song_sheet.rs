use anyhow::{Context, Result};
use clap::Parser;
use song_sheet::{config::Config, run};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Custom config file
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config"))]
    config: String,
}

fn main() -> Result<()> {
    // Init logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let cli = Cli::parse();

    let config = Config::read(&cli.config).with_context(|| "Error reading configuration.")?;
    run(&config).with_context(|| "Error while creating song sheet.")
}
