use std::sync::Mutex;

use rusqlite::Connection;
use serde::Serialize;
use tauri::{AppHandle, Manager, State};

use crate::domain::{EntityKind, NoteMeta};
use crate::error::{AppError, Result};
use crate::storage::{notes, notes_index, paths, tags};

/// A note plus its tag names — what the corkboard needs to render a card.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteCard {
    #[serde(flatten)]
    pub note: NoteMeta,
    pub tags: Vec<String>,
    pub preview: String,
}

fn card(conn: &Connection, note: NoteMeta) -> Result<NoteCard> {
    let tags = tags::names_for_entity(conn, EntityKind::Note, note.id)?;
    let preview = notes_index::preview(conn, note.id)?;
    Ok(NoteCard {
        note,
        tags,
        preview,
    })
}

/// Runs a storage call on the blocking pool. The note commands touch the file
/// system, and a synchronous command would do that on the main thread — stalling
/// the window and the connection lock that the tray and notification threads
/// also wait on.
async fn run_blocking<T, F>(app: AppHandle, work: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce(&Connection) -> Result<T> + Send + 'static,
{
    match tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<Mutex<Connection>>();
        let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
        work(&conn)
    })
    .await
    {
        Ok(result) => result,
        Err(error) => Err(AppError::InvalidInput(format!(
            "storage task failed: {error}"
        ))),
    }
}

#[tauri::command]
pub fn list_notes(state: State<'_, Mutex<Connection>>) -> Result<Vec<NoteCard>> {
    let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut cards = Vec::new();
    for note in notes_index::list(&conn)? {
        cards.push(card(&conn, note)?);
    }
    Ok(cards)
}

#[tauri::command]
pub async fn create_note(title: String, app: AppHandle) -> Result<NoteCard> {
    run_blocking(app, move |conn| {
        let note = notes::create(conn, &paths::notes_dir(), notes::NewNote { title })?;
        card(conn, note)
    })
    .await
}

#[tauri::command]
pub async fn note_body(id: i64, app: AppHandle) -> Result<String> {
    run_blocking(app, move |conn| notes::body(conn, &paths::notes_dir(), id)).await
}

#[tauri::command]
pub async fn save_note(
    id: i64,
    title: String,
    body: String,
    tags: Vec<String>,
    app: AppHandle,
) -> Result<NoteCard> {
    run_blocking(app, move |conn| {
        let note = notes::save(conn, &paths::notes_dir(), id, &title, &body, &tags)?;
        card(conn, note)
    })
    .await
}

#[tauri::command]
pub fn move_note(id: i64, x: f64, y: f64, state: State<'_, Mutex<Connection>>) -> Result<()> {
    notes_index::update_position(
        &state.lock().unwrap_or_else(|poison| poison.into_inner()),
        id,
        x,
        y,
    )
}

#[tauri::command]
pub async fn delete_note(id: i64, app: AppHandle) -> Result<()> {
    run_blocking(app, move |conn| {
        notes::delete(conn, &paths::notes_dir(), id)
    })
    .await
}

#[tauri::command]
pub fn search_notes(query: String, state: State<'_, Mutex<Connection>>) -> Result<Vec<i64>> {
    notes_index::search(
        &state.lock().unwrap_or_else(|poison| poison.into_inner()),
        &query,
    )
}
