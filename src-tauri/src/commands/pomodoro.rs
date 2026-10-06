use std::sync::Mutex;

use chrono::Duration;
use rusqlite::Connection;
use tauri::{AppHandle, Emitter, State};

use crate::domain::{PomodoroSession, ReminderKind, SessionKind};
use crate::error::Result;
use crate::reminder_sync;
use crate::storage::pomodoro::{self, PomodoroSettings};
use crate::storage::reminders;

#[tauri::command]
pub fn start_session(
    kind: SessionKind,
    task_id: Option<i64>,
    planned_seconds: i64,
    state: State<'_, Mutex<Connection>>,
) -> Result<PomodoroSession> {
    let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
    let session = pomodoro::start(&conn, kind, task_id, planned_seconds)?;
    announce_session_end(&conn, &session)?;
    Ok(session)
}

#[tauri::command]
pub fn finish_session(
    id: i64,
    completed: bool,
    state: State<'_, Mutex<Connection>>,
) -> Result<PomodoroSession> {
    let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
    let session = pomodoro::finish(&conn, id, completed)?;
    // However the session ended — completed, skipped, or stopped early — its
    // reminder is no longer something to announce.
    reminders::withdraw_for_session(&conn, session.id)?;
    reminder_sync::sync(&conn)?;
    Ok(session)
}

/// A work session's end is what a break reminder announces, and it is registered
/// with the OS as soon as the session starts rather than posted when it comes
/// due: the window is often behind something else by then.
fn announce_session_end(conn: &Connection, session: &PomodoroSession) -> Result<()> {
    if session.kind != SessionKind::Work {
        return Ok(());
    }

    let trigger_at = session.started_at + Duration::seconds(session.planned_seconds);
    reminders::create_for_session(conn, session.id, ReminderKind::PomodoroBreak, trigger_at)?;
    reminder_sync::sync(conn)
}

#[tauri::command]
pub fn reconcile_session(state: State<'_, Mutex<Connection>>) -> Result<Option<PomodoroSession>> {
    pomodoro::reconcile(&state.lock().unwrap_or_else(|poison| poison.into_inner()))
}

#[tauri::command]
pub fn next_phase(state: State<'_, Mutex<Connection>>) -> Result<SessionKind> {
    pomodoro::next_phase(&state.lock().unwrap_or_else(|poison| poison.into_inner()))
}

#[tauri::command]
pub fn cycle_position(state: State<'_, Mutex<Connection>>) -> Result<i64> {
    pomodoro::work_sessions_since_long_break(&state.lock().unwrap_or_else(|poison| poison.into_inner()))
}

#[tauri::command]
pub fn recent_sessions(
    limit: i64,
    state: State<'_, Mutex<Connection>>,
) -> Result<Vec<PomodoroSession>> {
    pomodoro::recent(&state.lock().unwrap_or_else(|poison| poison.into_inner()), limit)
}

#[tauri::command]
pub fn get_pomodoro_settings(state: State<'_, Mutex<Connection>>) -> Result<PomodoroSettings> {
    pomodoro::settings(&state.lock().unwrap_or_else(|poison| poison.into_inner()))
}

#[tauri::command]
pub fn set_pomodoro_settings(
    settings: PomodoroSettings,
    app: AppHandle,
    state: State<'_, Mutex<Connection>>,
) -> Result<PomodoroSettings> {
    let saved = {
        let conn = state.lock().unwrap_or_else(|poison| poison.into_inner());
        pomodoro::save_settings(&conn, &settings)?
    };

    // The durations are editable from two windows, so the other one has to hear
    // about it.
    let _ = app.emit(crate::SETTINGS_CHANGED, ());
    Ok(saved)
}
