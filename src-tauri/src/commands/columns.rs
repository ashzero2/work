use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::domain::{Column, NewColumn};
use crate::error::Result;
use crate::storage::columns;

#[tauri::command]
pub fn list_columns(state: State<'_, Mutex<Connection>>) -> Result<Vec<Column>> {
    columns::list(&state.lock().unwrap_or_else(|poison| poison.into_inner()))
}

#[tauri::command]
pub fn create_column(new_column: NewColumn, state: State<'_, Mutex<Connection>>) -> Result<Column> {
    columns::create(
        &state.lock().unwrap_or_else(|poison| poison.into_inner()),
        new_column,
    )
}

#[tauri::command]
pub fn rename_column(id: i64, name: String, state: State<'_, Mutex<Connection>>) -> Result<()> {
    columns::rename(
        &state.lock().unwrap_or_else(|poison| poison.into_inner()),
        id,
        &name,
    )
}

#[tauri::command]
pub fn delete_column(id: i64, state: State<'_, Mutex<Connection>>) -> Result<()> {
    columns::delete(
        &state.lock().unwrap_or_else(|poison| poison.into_inner()),
        id,
    )
}
