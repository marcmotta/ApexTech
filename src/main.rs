// src/main.rs
/*
 * Main executable for ApexTech
 */

use clap::Parser;
use apextech::{Result, run};

#[derive(Parser)]
#[command(version, about = "ApexTech - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
