mod commands;
pub mod domain;
pub mod error;
mod platform;
mod reminder_sync;
pub mod storage;

use std::sync::Mutex;

use chrono::{Duration, Utc};
use rusqlite::Connection;
use tauri::{Emitter, Manager};

use crate::domain::ReminderStatus;

/// Opens the database, seeds a default column, and runs the app.
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let conn = storage::db::open()?;
            storage::columns::ensure_default(&conn)?;

            // Settle anything that came due while the app was closed before
            // registering what is still outstanding.
            if let Err(error) = reminder_sync::reconcile(&conn) {
                eprintln!("reminder reconcile failed: {error}");
            }

            app.manage(Mutex::new(conn));

            let handle = app.handle().clone();
            platform::start_notifications(Box::new(move |action, identifier| {
                apply_notification_action(&handle, action, identifier);
            }));

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
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::pomodoro::start_session,
            commands::pomodoro::finish_session,
            commands::pomodoro::reconcile_session,
            commands::pomodoro::next_phase,
            commands::pomodoro::cycle_position,
            commands::pomodoro::recent_sessions,
            commands::pomodoro::get_pomodoro_settings,
            commands::pomodoro::set_pomodoro_settings,
            commands::reminders::list_reminders,
            commands::reminders::create_reminder,
            commands::reminders::reschedule_reminder,
            commands::reminders::snooze_reminder,
            commands::reminders::dismiss_reminder,
            commands::reminders::delete_reminder,
            commands::reminders::notification_state,
            commands::reminders::set_kind_enabled,
            commands::reminders::open_notification_settings,
            commands::system::system_accent,
            commands::system::set_traffic_lights_visible
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Maps a notification action back onto the reminder it came from. Called from
/// the OS delegate on the main thread, so it takes the same connection lock
/// every command does rather than holding one of its own.
fn apply_notification_action(app: &tauri::AppHandle, action: &str, identifier: &str) {
    eprintln!("notification action: {action} on {identifier}");

    let state = app.state::<Mutex<Connection>>();
    let conn = match state.lock() {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("notification action ignored: {error}");
            return;
        }
    };

    let reminder = match storage::reminders::find_by_tag(&conn, identifier) {
        Ok(Some(reminder)) => reminder,
        // Not one of ours, or already deleted.
        Ok(None) => return,
        Err(error) => {
            eprintln!("notification action failed: {error}");
            return;
        }
    };

    let result = if action == platform::ACTION_SNOOZE {
        let until = Utc::now() + Duration::minutes(storage::reminders::SNOOZE_MINUTES);
        storage::reminders::snooze(&conn, reminder.id, until).map(|_| ())
    } else if action == platform::ACTION_DISMISS {
        storage::reminders::mark(&conn, reminder.id, ReminderStatus::Dismissed).map(|_| ())
    } else {
        // Opening the notification settles it: the user has seen it.
        storage::reminders::mark(&conn, reminder.id, ReminderStatus::Fired).map(|_| ())
    };

    if let Err(error) = result {
        eprintln!("notification action failed: {error}");
        return;
    }

    // The action came from the OS rather than a command, so the window has no
    // way to know its state is now stale.
    let _ = app.emit(REMINDERS_CHANGED, ());
}

/// Emitted when a notification action changes reminder state behind the
/// frontend's back.
pub const REMINDERS_CHANGED: &str = "reminders:changed";
