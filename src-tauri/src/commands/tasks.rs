use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::domain::{NewTask, Task};
use crate::error::Result;
use crate::storage::tasks;

#[tauri::command]
pub fn list_tasks(state: State<'_, Mutex<Connection>>) -> Result<Vec<Task>> {
    tasks::list_all(&state.lock().unwrap())
}

#[tauri::command]
pub fn create_task(new_task: NewTask, state: State<'_, Mutex<Connection>>) -> Result<Task> {
    tasks::create(&state.lock().unwrap(), new_task)
}

#[tauri::command]
pub fn complete_task(id: i64, state: State<'_, Mutex<Connection>>) -> Result<Option<Task>> {
    tasks::complete_and_recur(&state.lock().unwrap(), id)
}

#[tauri::command]
pub fn delete_task(id: i64, state: State<'_, Mutex<Connection>>) -> Result<()> {
    tasks::delete(&state.lock().unwrap(), id)
}

#[tauri::command]
pub fn move_task_before(
    task_id: i64,
    column_id: i64,
    before_position: f64,
    state: State<'_, Mutex<Connection>>,
) -> Result<()> {
    tasks::move_before(&state.lock().unwrap(), task_id, column_id, before_position)
}

#[tauri::command]
pub fn move_task_to_end(
    task_id: i64,
    column_id: i64,
    state: State<'_, Mutex<Connection>>,
) -> Result<()> {
    tasks::move_to_end(&state.lock().unwrap(), task_id, column_id)
}
