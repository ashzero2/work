use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::domain::{PomodoroSession, SessionKind};
use crate::error::Result;
use crate::storage::pomodoro::{self, PomodoroSettings};

#[tauri::command]
pub fn start_session(
    kind: SessionKind,
    task_id: Option<i64>,
    planned_seconds: i64,
    state: State<'_, Mutex<Connection>>,
) -> Result<PomodoroSession> {
    pomodoro::start(&state.lock().unwrap(), kind, task_id, planned_seconds)
}

#[tauri::command]
pub fn finish_session(
    id: i64,
    completed: bool,
    state: State<'_, Mutex<Connection>>,
) -> Result<PomodoroSession> {
    pomodoro::finish(&state.lock().unwrap(), id, completed)
}

#[tauri::command]
pub fn reconcile_session(state: State<'_, Mutex<Connection>>) -> Result<Option<PomodoroSession>> {
    pomodoro::reconcile(&state.lock().unwrap())
}

#[tauri::command]
pub fn next_phase(state: State<'_, Mutex<Connection>>) -> Result<SessionKind> {
    pomodoro::next_phase(&state.lock().unwrap())
}

#[tauri::command]
pub fn cycle_position(state: State<'_, Mutex<Connection>>) -> Result<i64> {
    pomodoro::work_sessions_since_long_break(&state.lock().unwrap())
}

#[tauri::command]
pub fn recent_sessions(
    limit: i64,
    state: State<'_, Mutex<Connection>>,
) -> Result<Vec<PomodoroSession>> {
    pomodoro::recent(&state.lock().unwrap(), limit)
}

#[tauri::command]
pub fn get_pomodoro_settings(state: State<'_, Mutex<Connection>>) -> Result<PomodoroSettings> {
    pomodoro::settings(&state.lock().unwrap())
}

#[tauri::command]
pub fn set_pomodoro_settings(
    settings: PomodoroSettings,
    state: State<'_, Mutex<Connection>>,
) -> Result<PomodoroSettings> {
    pomodoro::save_settings(&state.lock().unwrap(), &settings)
}
