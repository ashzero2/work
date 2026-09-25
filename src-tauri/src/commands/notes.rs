use std::sync::Mutex;

use rusqlite::Connection;
use serde::Serialize;
use tauri::State;

use crate::domain::{EntityKind, NoteMeta};
use crate::error::Result;
use crate::storage::{notes, notes_index, paths, tags};

/// A note plus its tag names — what the corkboard needs to render a card.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteCard {
    #[serde(flatten)]
    pub note: NoteMeta,
    pub tags: Vec<String>,
}

fn card(conn: &Connection, note: NoteMeta) -> Result<NoteCard> {
    let tags = tags::names_for_entity(conn, EntityKind::Note, note.id)?;
    Ok(NoteCard { note, tags })
}

#[tauri::command]
pub fn list_notes(state: State<'_, Mutex<Connection>>) -> Result<Vec<NoteCard>> {
    let conn = state.lock().unwrap();
    let mut cards = Vec::new();
    for note in notes_index::list(&conn)? {
        cards.push(card(&conn, note)?);
    }
    Ok(cards)
}

#[tauri::command]
pub fn create_note(title: String, state: State<'_, Mutex<Connection>>) -> Result<NoteCard> {
    let conn = state.lock().unwrap();
    let note = notes::create(&conn, &paths::notes_dir(), notes::NewNote { title })?;
    card(&conn, note)
}

#[tauri::command]
pub fn note_body(id: i64, state: State<'_, Mutex<Connection>>) -> Result<String> {
    notes::body(&state.lock().unwrap(), &paths::notes_dir(), id)
}

#[tauri::command]
pub fn save_note(
    id: i64,
    title: String,
    body: String,
    tags: Vec<String>,
    state: State<'_, Mutex<Connection>>,
) -> Result<NoteCard> {
    let conn = state.lock().unwrap();
    let note = notes::save(&conn, &paths::notes_dir(), id, &title, &body, &tags)?;
    card(&conn, note)
}

#[tauri::command]
pub fn move_note(id: i64, x: f64, y: f64, state: State<'_, Mutex<Connection>>) -> Result<()> {
    notes_index::update_position(&state.lock().unwrap(), id, x, y)
}

#[tauri::command]
pub fn delete_note(id: i64, state: State<'_, Mutex<Connection>>) -> Result<()> {
    notes::delete(&state.lock().unwrap(), &paths::notes_dir(), id)
}

#[tauri::command]
pub fn search_notes(query: String, state: State<'_, Mutex<Connection>>) -> Result<Vec<i64>> {
    notes_index::search(&state.lock().unwrap(), &query)
}
