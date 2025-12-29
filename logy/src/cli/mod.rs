mod probe;

use anyhow::Result;
use clap::{Parser, Subcommand};
use probe::ProbeCommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    color: colorchoice_clap::Color,

    #[command(subcommand)]
    command: Commands,

    /// Output plain JSON without color and interactivity
    #[arg(short, long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    Probe(ProbeCommand),
}

pub async fn execute() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing subscriber.
    // The EnvFilter allows configuring the log level via the RUST_LOG environment variable, e.g.
    // RUST_LOG=debug or RUST_LOG=hidpp=trace.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();

    cli.color.write_global();

    match &cli.command {
        Commands::Probe(cmd) => cmd.execute(&cli).await,
    }
}
