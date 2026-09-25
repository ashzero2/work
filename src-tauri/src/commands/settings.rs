use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::Result;
use crate::storage::settings;

#[tauri::command]
pub fn get_setting(key: String, state: State<'_, Mutex<Connection>>) -> Result<Option<String>> {
    settings::get(&state.lock().unwrap(), &key)
}

#[tauri::command]
pub fn set_setting(key: String, value: String, state: State<'_, Mutex<Connection>>) -> Result<()> {
    settings::set(&state.lock().unwrap(), &key, &value)
}
