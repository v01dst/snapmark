use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name="snapmark", version, about="Local-first bookmarks and snippets")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add { name: String, value: String, #[arg(short, long, value_delimiter=',')] tags: Vec<String> },
    List { #[arg(short, long)] tag: Option<String> },
    Search { query: String },
    Get { name: String },
    Rm { name: String },
    Rename { old: String, new: String },
    Tag { name: String, tags: Vec<String> },
    Clear,
}
