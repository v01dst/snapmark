use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{model::Entry, search, storage};

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/entries", get(list).post(add))
        .route("/api/entries/{name}", delete(remove))
        .route("/api/search", get(search_entries))
}

async fn index() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("content-type", "text/html; charset=utf-8")],
        include_str!("../web/index.html"),
    )
}

async fn list() -> impl IntoResponse {
    match storage::load() {
        Ok(store) => Json(store.entries).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn search_entries(Query(query): Query<SearchQuery>) -> impl IntoResponse {
    let q = query.q.unwrap_or_default();
    match storage::load() {
        Ok(store) => Json(
            store
                .entries
                .into_iter()
                .filter(|e| search::matches(e, &q))
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn add(Json(entry): Json<Entry>) -> impl IntoResponse {
    match storage::load().and_then(|mut store| {
        storage::upsert(&mut store, entry)?;
        storage::save(&store)?;
        Ok(())
    }) {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn remove(Path(name): Path<String>) -> impl IntoResponse {
    match storage::load().and_then(|mut store| {
        let before = store.entries.len();
        store.entries.retain(|e| e.name != name);
        if before == store.entries.len() {
            return Err(crate::error::Error::NotFound(name));
        }
        storage::save(&store)
    }) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(crate::error::Error::NotFound(_)) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
