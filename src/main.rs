mod cli;
mod commands;
mod error;
mod model;
mod storage;
mod search;

use clap::Parser;
use cli::Cli;
use error::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli.command)
}
