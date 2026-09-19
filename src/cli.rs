use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "snapmark", version, about = "Local-first developer knowledge vault")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add {
        name: String,
        value: String,
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },
    List {
        #[arg(short, long)]
        tag: Option<String>,
    },
    Search {
        query: String,
    },
    Get {
        name: String,
    },
    Rm {
        name: String,
    },
    Rename {
        old: String,
        new: String,
    },
    Tag {
        name: String,
        #[arg(value_delimiter = ',')]
        tags: Vec<String>,
    },
    Stats,
    Path,
    Export {
        file: String,
    },
    Import {
        file: String,
        #[arg(long)]
        replace: bool,
    },
    Clear {
        #[arg(long)]
        yes: bool,
    },
    Serve {
        #[arg(short, long, default_value_t = 8787)]
        port: u16,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long)]
        open: bool,
    },
}
