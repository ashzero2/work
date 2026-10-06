use std::sync::Mutex;

use rusqlite::Connection;
use tauri::{AppHandle, Emitter, State};

use crate::error::Result;
use crate::storage::settings;

#[tauri::command]
pub fn get_setting(key: String, state: State<'_, Mutex<Connection>>) -> Result<Option<String>> {
    settings::get(&state.lock().unwrap_or_else(|poison| poison.into_inner()), &key)
}

/// Writes a preference and tells every window about it.
///
/// Emitted from here rather than from each specific setting, because the main
/// window and the settings window are separate documents that cannot see each
/// other's state — and because it means a preference added later is announced
/// without anyone remembering to announce it.
#[tauri::command]
pub fn set_setting(
    key: String,
    value: String,
    app: AppHandle,
    state: State<'_, Mutex<Connection>>,
) -> Result<()> {
    {
        let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
        settings::set(&conn, &key, &value)?;
    }

    let _ = app.emit(crate::SETTINGS_CHANGED, ());
    Ok(())
}
