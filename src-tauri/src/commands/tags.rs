use std::sync::Mutex;

use rusqlite::Connection;
use serde::Serialize;
use tauri::State;

use crate::domain::EntityKind;
use crate::error::Result;
use crate::storage::tags;

/// A tag with the number of notes carrying it, for the sidebar list.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagSummary {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub note_count: i64,
}

#[tauri::command]
pub fn list_tags(state: State<'_, Mutex<Connection>>) -> Result<Vec<TagSummary>> {
    let conn = state.lock().unwrap();
    let summaries = tags::list_with_counts(&conn, EntityKind::Note)?
        .into_iter()
        .map(|(tag, note_count)| TagSummary {
            id: tag.id,
            name: tag.name,
            color: tag.color,
            note_count,
        })
        .collect();
    Ok(summaries)
}

#[tauri::command]
pub fn delete_tag(id: i64, state: State<'_, Mutex<Connection>>) -> Result<()> {
    tags::delete(&state.lock().unwrap(), id)
}
