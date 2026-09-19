mod cli;
mod commands;
mod error;
mod model;
mod search;
mod storage;
mod web;

use clap::Parser;
use cli::Cli;
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli.command).await
}
