use crate::{cli::Command, error::{Error, Result}, model::Entry, search, storage};

pub fn run(command: Command) -> Result<()> {
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
            let e = store.entries.iter().find(|e| e.name == name).ok_or_else(|| Error::NotFound(name.clone()))?;
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
            if store.entries.iter().any(|e| e.name == new) { return Err(Error::AlreadyExists(new)); }
            let e = store.entries.iter_mut().find(|e| e.name == old).ok_or_else(|| Error::NotFound(old.clone()))?;
            e.name = new;
            storage::save(&store)?;
            println!("renamed.");
        }
        Command::Tag { name, tags } => {
            let e = store.entries.iter_mut().find(|e| e.name == name).ok_or_else(|| Error::NotFound(name.clone()))?;
            e.tags = tags;
            storage::save(&store)?;
            println!("tags updated.");
        }
        Command::Clear => {
            store.entries.clear();
            storage::save(&store)?;
            println!("cleared.");
        }
    }
    Ok(())
}

fn print_entry(e: &Entry) {
    if e.tags.is_empty() { println!("{}  {}", e.name, e.value); }
    else { println!("{}  {}  [{}]", e.name, e.value, e.tags.join(", ")); }
}
