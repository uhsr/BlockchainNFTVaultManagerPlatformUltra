// src/main.rs
/*
 * Main executable for BlockchainNFTVaultManagerPlatformUltra
 */

use clap::Parser;
use blockchainnftvaultmanagerplatformultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTVaultManagerPlatformUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
