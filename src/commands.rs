use crate::{cli::Command, error::{Error, Result}, model::{Entry, Store}, search, storage};
use std::{fs, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn run(command: Command) -> Result<()> {
    match command {
        Command::Serve { port, host, open } => serve(&host, port, open).await,
        command => run_local(command),
    }
}

fn run_local(command: Command) -> Result<()> {
    let mut store = storage::load()?;
    match command {
        Command::Add { name, value, tags } => {
            storage::upsert(&mut store, Entry { name, value, tags })?;
            storage::save(&store)?;
            println!("saved.");
        }
        Command::List { tag } => {
            for e in store.entries.iter().filter(|e| tag.as_ref().map_or(true, |t| e.tags.iter().any(|x| x == t))) {
                print_entry(e);
            }
        }
        Command::Search { query } => {
            for e in store.entries.iter().filter(|e| search::matches(e, &query)) { print_entry(e); }
        }
        Command::Get { name } => {
            let e = find(&store, &name)?;
            println!("{}", e.value);
        }
        Command::Rm { name } => {
            let before = store.entries.len();
            store.entries.retain(|e| e.name != name);
            if before == store.entries.len() { return Err(Error::NotFound(name)); }
            storage::save(&store)?;
            println!("removed.");
        }
        Command::Rename { old, new } => {
            if new.trim().is_empty() { return Err(Error::InvalidInput("new name cannot be empty".into())); }
            if store.entries.iter().any(|e| e.name == new) { return Err(Error::AlreadyExists(new)); }
            let e = store.entries.iter_mut().find(|e| e.name == old).ok_or_else(|| Error::NotFound(old.clone()))?;
            e.name = new;
            storage::save(&store)?;
            println!("renamed.");
        }
        Command::Tag { name, tags } => {
            let e = store.entries.iter_mut().find(|e| e.name == name).ok_or_else(|| Error::NotFound(name.clone()))?;
            e.tags = tags;
            store.normalize();
            storage::save(&store)?;
            println!("tags updated.");
        }
        Command::Stats => {
            let tags: usize = store.entries.iter().map(|e| e.tags.len()).sum();
            println!("entries: {}", store.entries.len());
            println!("tags: {}", tags);
        }
        Command::Path => println!("{}", storage::path().display()),
        Command::Export { file } => export_json(&store, &file)?,
        Command::Import { file, replace } => {
            let raw = fs::read_to_string(&file)?;
            let mut incoming: Store = serde_json::from_str(&raw)?;
            incoming.normalize();
            if replace { store = incoming; } else {
                for entry in incoming.entries { storage::upsert(&mut store, entry)?; }
            }
            storage::save(&store)?;
            println!("imported.");
        }
        Command::Clear { yes } => {
            if !yes { return Err(Error::InvalidInput("clear is destructive; re-run with --yes".into())); }
            store.entries.clear();
            storage::save(&store)?;
            println!("cleared.");
        }
        Command::Serve { .. } => unreachable!(),
    }
    Ok(())
}

fn find<'a>(store: &'a Store, name: &str) -> Result<&'a Entry> {
    store.entries.iter().find(|e| e.name == name).ok_or_else(|| Error::NotFound(name.to_owned()))
}

fn export_json(store: &Store, file: &str) -> Result<()> {
    fs::write(file, serde_json::to_string_pretty(store)?)?;
    println!("exported to {file}.");
    Ok(())
}

async fn serve(host: &str, port: u16, _open: bool) -> Result<()> {
    let app = crate::web::router();
    let addr: SocketAddr = format!("{host}:{port}").parse().map_err(|e| Error::Server(e.to_string()))?;
    println!("snapmark web → http://{addr}");
    let listener = TcpListener::bind(addr).await.map_err(|e| Error::Server(e.to_string()))?;
    axum::serve(listener, app).await.map_err(|e| Error::Server(e.to_string()))
}

fn print_entry(e: &Entry) {
    if e.tags.is_empty() { println!("{}  {}", e.name, e.value); }
    else { println!("{}  {}  [{}]", e.name, e.value, e.tags.join(", ")); }
}
