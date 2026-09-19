use crate::{error::Result, model::{Entry, Store}};
use std::{fs, path::PathBuf};

pub fn path() -> PathBuf {
    dirs::data_local_dir().unwrap_or_else(|| PathBuf::from(".")).join("snapmark/store.json")
}

pub fn load() -> Result<Store> {
    let p = path();
    if !p.exists() { return Ok(Store::default()); }
    Ok(serde_json::from_str(&fs::read_to_string(p)?)?)
}

pub fn save(store: &Store) -> Result<()> {
    let p = path();
    if let Some(parent) = p.parent() { fs::create_dir_all(parent)?; }
    fs::write(p, serde_json::to_string_pretty(store)?)?;
    Ok(())
}

pub fn upsert(store: &mut Store, entry: Entry) -> Result<()> {
    if let Some(existing) = store.entries.iter_mut().find(|e| e.name == entry.name) {
        *existing = entry;
    } else {
        store.entries.push(entry);
    }
    Ok(())
}
