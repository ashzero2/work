use std::sync::Mutex;

use rusqlite::Connection;
use serde::Deserialize;
use tauri::State;
use tauri_plugin_dialog::DialogExt;

use crate::error::{AppError, Result};
use crate::export;

/// The formats on offer, parsed rather than taken as a free-form extension so an
/// unknown request is rejected instead of silently writing the wrong encoding.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Json,
    Csv,
}

impl ExportFormat {
    fn file_name(self) -> &'static str {
        match self {
            ExportFormat::Json => "tasks.json",
            ExportFormat::Csv => "tasks.csv",
        }
    }

    fn filter(self) -> (&'static str, &'static str) {
        match self {
            ExportFormat::Json => ("JSON", "json"),
            ExportFormat::Csv => ("CSV", "csv"),
        }
    }

    fn encode(self, rows: &[export::ExportTask]) -> Result<String> {
        match self {
            ExportFormat::Json => export::to_json(rows),
            ExportFormat::Csv => Ok(export::to_csv(rows)),
        }
    }
}

/// Writes every task to a file the user picks. Async because the save dialog must
/// not run on the main thread, and the rows are collected and the connection
/// released *before* the dialog opens — a `std::sync` guard must never be held
/// across an await.
#[tauri::command]
pub async fn export_tasks(
    format: ExportFormat,
    app: tauri::AppHandle,
    state: State<'_, Mutex<Connection>>,
) -> Result<Option<String>> {
    let contents = {
        let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
        format.encode(&export::tasks(&conn)?)?
    };

    let chosen = app
        .dialog()
        .file()
        .set_title("Export tasks")
        .set_file_name(format.file_name())
        .add_filter(format.filter().0, &[format.filter().1])
        .blocking_save_file();

    let Some(chosen) = chosen else {
        // Cancelled: not an error, and nothing to write.
        return Ok(None);
    };

    let path = chosen
        .into_path()
        .map_err(|error| AppError::InvalidInput(error.to_string()))?;

    // Plain disk IO, so it goes on the blocking pool rather than holding up the
    // async command's thread.
    let target = path.clone();
    tauri::async_runtime::spawn_blocking(move || std::fs::write(&target, contents))
        .await
        .map_err(|error| AppError::InvalidInput(format!("export task failed: {error}")))??;

    Ok(Some(path.display().to_string()))
}
