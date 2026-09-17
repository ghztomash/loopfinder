use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;
use loopfinder_video::decoder::VideoDecoder;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, filter::LevelFilter};

use loopfinder_core::detector::{DetectorConfig, LoopDetecor};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    input: PathBuf,

    #[arg(long, default_value_t = 5)]
    sample_fps: usize,

    #[arg(long, default_value_t = 1.0)]
    min_loop: f64,

    #[arg(long, default_value_t = 10.0)]
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

    let config = DetectorConfig {
        min_duration: cli.min_loop,
        max_duration: cli.max_loop,
        sequence_length: cli.sample_fps,
    };

    let video = VideoDecoder::open(&cli.input)?;
    debug!("Loaded {:?}", video.metadata());
    // TODO: decode and extract frames in provided scale
    // frames .. video.next_frames()

    // let features = extract_frames()?;
    let detector = LoopDetecor::new(config);
    let candidates = detector.detect(&[]).unwrap();

    debug!("candidates {:?}", candidates);

    Ok(())
}
