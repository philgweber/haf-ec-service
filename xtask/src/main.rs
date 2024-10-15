use anyhow::Result;
use clap::Parser;
use xtask::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    Cli::parse().run().await
}
