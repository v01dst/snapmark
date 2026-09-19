use crate::{error::{Error, Result}, model::{Entry, Store}};
use std::{fs, io::Write, path::PathBuf};

pub fn path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("snapmark")
        .join("store.json")
}

pub fn load() -> Result<Store> {
    let p = path();
    if !p.exists() { return Ok(Store::default()); }
    let raw = fs::read_to_string(p)?;
    let mut store: Store = serde_json::from_str(&raw)?;
    store.normalize();
    Ok(store)
}

pub fn save(store: &Store) -> Result<()> {
    let p = path();
    if let Some(parent) = p.parent() { fs::create_dir_all(parent)?; }
    let tmp = p.with_extension("json.tmp");
    let mut file = fs::File::create(&tmp)?;
    file.write_all(serde_json::to_string_pretty(store)?.as_bytes())?;
    file.sync_all()?;
    fs::rename(tmp, p)?;
    Ok(())
}

pub fn upsert(store: &mut Store, entry: Entry) -> Result<()> {
    let name = entry.name.trim();
    if name.is_empty() { return Err(Error::InvalidInput("name cannot be empty".into())); }
    if let Some(existing) = store.entries.iter_mut().find(|e| e.name == name) {
        *existing = entry;
    } else {
        store.entries.push(entry);
    }
    store.normalize();
    Ok(())
}
