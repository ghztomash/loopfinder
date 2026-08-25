use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, filter::LevelFilter};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    input: PathBuf,

    #[arg(long)]
    sample_fps: usize,

    #[arg(long)]
    min_loop: f64,

    #[arg(long)]
    max_loop: f64,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn log_level(verbose: u8) -> LevelFilter {
    match verbose {
        0 => LevelFilter::WARN,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    }
}

fn init_logging(verbose: u8) {
    let filter = EnvFilter::builder()
        .with_default_directive(log_level(verbose).into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    init_logging(cli.verbose);

    info!(input = %cli.input.display(), "starting loopfinder");
    debug!(verbosity = cli.verbose, "verbosity level set");

    Ok(())
}
