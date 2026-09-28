use std::sync::Mutex;

use rusqlite::Connection;
use tauri::{AppHandle, Emitter, State, WebviewWindow};

use crate::domain::{NewTask, Priority};
use crate::error::Result;
use crate::storage::{columns, tasks};

/// The user's system accent colour, so the platform theme can mirror it. Kept
/// deliberately narrow: the frontend detects the platform itself, so a failed
/// colour read can never masquerade as "not macOS".
#[tauri::command]
pub fn system_accent() -> Option<String> {
    crate::platform::accent_color()
}

/// Matches the window controls to the sidebar being collapsed. Showing them
/// again makes AppKit re-lay the title bar out, so the alignment is re-applied
/// afterwards rather than being left at the stock position.
#[tauri::command]
pub fn set_traffic_lights_visible(window: WebviewWindow, visible: bool) {
    crate::platform::set_traffic_lights_visible(&window, visible);
    crate::platform::align_traffic_lights_soon(&window);
}

/// Quick capture: files a task into the first column and dismisses the capture
/// window. One command rather than a create-then-hide pair, so the window cannot
/// vanish before the task is actually stored.
#[tauri::command]
pub fn capture_task(
    title: String,
    app: AppHandle,
    state: State<'_, Mutex<Connection>>,
) -> Result<()> {
    let title = title.trim().to_string();
    if title.is_empty() {
        crate::quick_capture::hide(&app);
        return Ok(());
    }

    {
        let conn = state.lock().unwrap();
        let column = columns::ensure_default(&conn)?;
        tasks::create(
            &conn,
            NewTask {
                title,
                description: None,
                column_id: column.id,
                priority: Priority::None,
                due_at: None,
                repeat_rule: None,
                parent_task_id: None,
            },
        )?;
    }

    crate::quick_capture::hide(&app);
    // Released before announcing, so the listener's own commands can take the
    // connection lock without waiting on us.
    let _ = app.emit(crate::TASKS_CHANGED, ());
    Ok(())
}

/// Dismisses the capture window without storing anything — Escape, and clicking
/// away. A command rather than a window call from the frontend, so the capture
/// window needs no window permissions of its own.
#[tauri::command]
pub fn hide_capture_window(app: AppHandle) {
    crate::quick_capture::hide(&app);
}

/// The quick-capture shortcut as configured, along with what is actually bound —
/// the two differ when the stored combination was refused at startup.
#[tauri::command]
pub fn get_quick_capture_shortcut(app: AppHandle) -> crate::quick_capture::ShortcutStatus {
    crate::quick_capture::status(&app)
}

#[tauri::command]
pub fn set_quick_capture_shortcut(
    shortcut: String,
    app: AppHandle,
    state: State<'_, Mutex<Connection>>,
) -> Result<crate::quick_capture::ShortcutStatus> {
    crate::quick_capture::set_preference(&app, &state.lock().unwrap(), &shortcut)
}
