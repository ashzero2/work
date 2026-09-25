use rusqlite::{Connection, OptionalExtension, params};

use crate::domain::{Column, NewColumn};
use crate::error::{AppError, Result};

pub fn create(conn: &Connection, new_column: NewColumn) -> Result<Column> {
    let order_index: f64 = conn.query_row(
        "SELECT COALESCE(MAX(order_index), 0.0) + 1.0 FROM columns",
        [],
        |row| row.get(0),
    )?;

    conn.execute(
        "INSERT INTO columns (name, order_index, color, wip_limit) VALUES (?1, ?2, ?3, ?4)",
        params![
            new_column.name,
            order_index,
            new_column.color,
            new_column.wip_limit
        ],
    )?;

    let id = conn.last_insert_rowid();
    get(conn, id)?.ok_or(AppError::NotFound)
}

/// Returns the first column, creating a default "Tasks" column if none
/// exist yet. Used before column management UI exists (Phase 1), so a task
/// always has somewhere to go.
pub fn ensure_default(conn: &Connection) -> Result<Column> {
    let existing = list(conn)?;
    if let Some(first) = existing.into_iter().next() {
        return Ok(first);
    }
    create(
        conn,
        NewColumn {
            name: "Tasks".into(),
            color: None,
            wip_limit: None,
        },
    )
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<Column>> {
    conn.query_row(
        "SELECT id, name, order_index, color, wip_limit FROM columns WHERE id = ?1",
        params![id],
        row_to_column,
    )
    .optional()
    .map_err(Into::into)
}

pub fn list(conn: &Connection) -> Result<Vec<Column>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, order_index, color, wip_limit FROM columns ORDER BY order_index",
    )?;
    let rows = stmt.query_map([], row_to_column)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

pub fn rename(conn: &Connection, id: i64, name: &str) -> Result<()> {
    conn.execute(
        "UPDATE columns SET name = ?1 WHERE id = ?2",
        params![name, id],
    )?;
    Ok(())
}

/// Deletes a column. Refuses if it still has tasks — callers must move
/// tasks out first; columns are never auto-migrated or cascade-deleted.
pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    let task_count: i64 = conn.query_row(
        "SELECT count(*) FROM tasks WHERE column_id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    if task_count > 0 {
        return Err(AppError::ColumnNotEmpty);
    }
    conn.execute("DELETE FROM columns WHERE id = ?1", params![id])?;
    Ok(())
}

fn row_to_column(row: &rusqlite::Row) -> rusqlite::Result<Column> {
    Ok(Column {
        id: row.get(0)?,
        name: row.get(1)?,
        order_index: row.get(2)?,
        color: row.get(3)?,
        wip_limit: row.get(4)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{NewTask, Priority};
    use crate::storage::{db, tasks};

    fn new_column(name: &str) -> NewColumn {
        NewColumn {
            name: name.into(),
            color: None,
            wip_limit: None,
        }
    }

    #[test]
    fn create_list_and_rename() {
        let conn = db::open_in_memory().unwrap();
        let a = create(&conn, new_column("Todo")).unwrap();
        create(&conn, new_column("Doing")).unwrap();

        let all = list(&conn).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].name, "Todo");

        rename(&conn, a.id, "Backlog").unwrap();
        assert_eq!(get(&conn, a.id).unwrap().unwrap().name, "Backlog");
    }

    #[test]
    fn delete_is_blocked_while_column_has_tasks() {
        let conn = db::open_in_memory().unwrap();
        let column = create(&conn, new_column("Todo")).unwrap();
        tasks::create(
            &conn,
            NewTask {
                title: "A task".into(),
                description: None,
                column_id: column.id,
                priority: Priority::None,
                due_at: None,
                repeat_rule: None,
                parent_task_id: None,
            },
        )
        .unwrap();

        let err = delete(&conn, column.id).unwrap_err();
        assert!(matches!(err, AppError::ColumnNotEmpty));
    }

    #[test]
    fn ensure_default_creates_once_then_reuses_it() {
        let conn = db::open_in_memory().unwrap();
        let first = ensure_default(&conn).unwrap();
        assert_eq!(first.name, "Tasks");

        let second = ensure_default(&conn).unwrap();
        assert_eq!(second.id, first.id);
        assert_eq!(list(&conn).unwrap().len(), 1);
    }
}
