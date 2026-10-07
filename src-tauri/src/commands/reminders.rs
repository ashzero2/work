use std::sync::Mutex;

use chrono::{DateTime, Duration, Utc};
use rusqlite::Connection;
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use crate::domain::{Reminder, ReminderKind, ReminderStatus};
use crate::error::Result;
use crate::reminder_sync;
use crate::storage::reminders::{self, NewReminder, SNOOZE_MINUTES};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationState {
    /// Whether the OS can deliver notifications at all — false under `tauri dev`,
    /// where the process has no bundle identifier, and off macOS entirely.
    pub available: bool,
    /// The answer to the permission prompt, or `None` if it hasn't been asked.
    pub authorized: Option<bool>,
    pub enabled: Vec<KindToggle>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KindToggle {
    pub kind: ReminderKind,
    pub enabled: bool,
}

#[tauri::command]
pub fn list_reminders(state: State<'_, Mutex<Connection>>) -> Result<Vec<Reminder>> {
    reminders::list(&state.lock().unwrap_or_else(|poison| poison.into_inner()))
}

#[tauri::command]
pub fn create_reminder(
    kind: ReminderKind,
    task_id: Option<i64>,
    trigger_at: DateTime<Utc>,
    state: State<'_, Mutex<Connection>>,
) -> Result<Reminder> {
    let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
    let reminder = reminders::create(
        &conn,
        NewReminder {
            kind,
            task_id,
            session_id: None,
            trigger_at,
        },
    )?;
    reminder_sync::sync(&conn)?;
    Ok(reminder)
}

#[tauri::command]
pub fn reschedule_reminder(
    id: i64,
    trigger_at: DateTime<Utc>,
    state: State<'_, Mutex<Connection>>,
) -> Result<Reminder> {
    let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
    let reminder = reminders::reschedule(&conn, id, trigger_at)?;
    reminder_sync::sync(&conn)?;
    Ok(reminder)
}

#[tauri::command]
pub fn snooze_reminder(id: i64, state: State<'_, Mutex<Connection>>) -> Result<Reminder> {
    let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
    let until = Utc::now() + Duration::minutes(SNOOZE_MINUTES);
    let reminder = reminders::snooze(&conn, id, until)?;
    reminder_sync::sync(&conn)?;
    Ok(reminder)
}

#[tauri::command]
pub fn dismiss_reminder(id: i64, state: State<'_, Mutex<Connection>>) -> Result<Reminder> {
    let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
    let reminder = reminders::mark(&conn, id, ReminderStatus::Dismissed)?;
    reminder_sync::sync(&conn)?;
    Ok(reminder)
}

#[tauri::command]
pub fn delete_reminder(id: i64, state: State<'_, Mutex<Connection>>) -> Result<()> {
    let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
    reminders::delete(&conn, id)?;
    reminder_sync::sync(&conn)
}

#[tauri::command]
pub fn notification_state(state: State<'_, Mutex<Connection>>) -> Result<NotificationState> {
    let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());

    Ok(NotificationState {
        available: crate::platform::notifications_available(),
        authorized: crate::platform::notifications_authorized(),
        enabled: ReminderKind::ALL
            .into_iter()
            .map(|kind| {
                Ok(KindToggle {
                    kind,
                    enabled: reminders::enabled(&conn, kind)?,
                })
            })
            .collect::<Result<Vec<_>>>()?,
    })
}

#[tauri::command]
pub fn set_kind_enabled(
    kind: ReminderKind,
    enabled: bool,
    app: AppHandle,
    state: State<'_, Mutex<Connection>>,
) -> Result<()> {
    {
        let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
        reminders::set_enabled(&conn, kind, enabled)?;
        reminder_sync::sync(&conn)?;
    }

    // The toggles are editable from two windows, so the other one has to hear
    // about it.
    let _ = app.emit(crate::SETTINGS_CHANGED, ());
    Ok(())
}

/// The recovery path when permission was refused: send the user where they can
/// grant it, rather than leaving reminders silently doing nothing.
#[tauri::command]
pub fn open_notification_settings() {
    crate::platform::open_notification_settings();
}
