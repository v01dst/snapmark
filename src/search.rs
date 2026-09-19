use crate::model::Entry;

pub fn matches(entry: &Entry, query: &str) -> bool {
    let q = query.to_lowercase();
    entry.name.to_lowercase().contains(&q)
        || entry.value.to_lowercase().contains(&q)
        || entry.tags.iter().any(|t| t.to_lowercase().contains(&q))
}
