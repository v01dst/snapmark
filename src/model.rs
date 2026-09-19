use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Store {
    #[serde(default)]
    pub entries: Vec<Entry>,
}

impl Store {
    pub fn normalize(&mut self) {
        for entry in &mut self.entries {
            entry.tags.retain(|t| !t.trim().is_empty());
            entry.tags.sort();
            entry.tags.dedup();
        }
    }
}
