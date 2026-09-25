mod commands;
pub mod domain;
pub mod error;
mod platform;
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

            match app.get_webview_window("main") {
                Some(window) => {
                    platform::apply_window_material(&window);
                    platform::align_traffic_lights_soon(&window);

                    // AppKit re-lays the controls out on resize, so the alignment
                    // is re-applied rather than set once.
                    let handle = app.handle().clone();
                    window.on_window_event(move |event| {
                        if matches!(
                            event,
                            tauri::WindowEvent::Resized(_)
                                | tauri::WindowEvent::ScaleFactorChanged { .. }
                        ) && let Some(window) = handle.get_webview_window("main")
                        {
                            platform::align_traffic_lights_soon(&window);
                        }
                    });
                }
                None => eprintln!("main window not found; window material not applied"),
            }
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
            commands::tags::delete_tag,
            commands::system::system_accent,
            commands::system::set_traffic_lights_visible
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
