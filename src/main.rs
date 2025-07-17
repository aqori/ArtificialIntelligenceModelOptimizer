// src/main.rs
/*
 * Main executable for ArtificialIntelligenceModelOptimizer
 */

use clap::Parser;
use artificialintelligencemodeloptimizer::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialIntelligenceModelOptimizer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
