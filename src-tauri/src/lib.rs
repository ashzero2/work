mod commands;
pub mod domain;
pub mod error;
pub mod storage;

use std::sync::Mutex;

use tauri::Manager;

/// Opens the database, seeds a default column, and runs the app.
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let conn = storage::db::open()?;
            storage::columns::ensure_default(&conn)?;
            app.manage(Mutex::new(conn));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::tasks::list_tasks,
            commands::tasks::create_task,
            commands::tasks::complete_task,
            commands::tasks::delete_task,
            commands::tasks::move_task_before,
            commands::tasks::move_task_to_end,
            commands::columns::list_columns,
            commands::columns::create_column,
            commands::columns::rename_column,
            commands::columns::delete_column,
            commands::notes::list_notes,
            commands::notes::create_note,
            commands::notes::note_body,
            commands::notes::save_note,
            commands::notes::move_note,
            commands::notes::delete_note,
            commands::notes::search_notes,
            commands::tags::list_tags,
            commands::tags::delete_tag
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
