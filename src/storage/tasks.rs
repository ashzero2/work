use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::domain::{NewTask, Priority, RepeatRule, Task};
use crate::error::{AppError, Result};

pub fn create(conn: &Connection, new_task: NewTask) -> Result<Task> {
    let now = Utc::now().to_rfc3339();
    let position: f64 = conn.query_row(
        "SELECT COALESCE(MAX(position), 0.0) + 1.0 FROM tasks WHERE column_id = ?1",
        params![new_task.column_id],
        |row| row.get(0),
    )?;

    conn.execute(
        "INSERT INTO tasks
            (title, description, column_id, position, priority, due_at, repeat_rule, parent_task_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
        params![
            new_task.title,
            new_task.description,
            new_task.column_id,
            position,
            new_task.priority.as_i64(),
            new_task.due_at.map(|d| d.to_rfc3339()),
            new_task.repeat_rule.map(RepeatRule::as_str),
            new_task.parent_task_id,
            now,
        ],
    )?;

    let id = conn.last_insert_rowid();
    get(conn, id)?.ok_or(AppError::NotFound)
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<Task>> {
    conn.query_row(
        &SELECT_TASK.replace("{filter}", "WHERE id = ?1"),
        params![id],
        row_to_task,
    )
    .optional()
    .map_err(Into::into)
}

pub fn list_by_column(conn: &Connection, column_id: i64) -> Result<Vec<Task>> {
    let sql = SELECT_TASK.replace("{filter}", "WHERE column_id = ?1 ORDER BY position");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![column_id], row_to_task)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Every task across every column — the board and the list view both start
/// from this same query, then render it differently.
pub fn list_all(conn: &Connection) -> Result<Vec<Task>> {
    let sql = SELECT_TASK.replace("{filter}", "ORDER BY created_at");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_to_task)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Moves a task into `column_id`, inserting it directly before whichever
/// task currently holds `before_position` in that column — fractional
/// ranking, so no sibling ever needs renumbering.
pub fn move_before(conn: &Connection, id: i64, column_id: i64, before_position: f64) -> Result<()> {
    let prev_position: Option<f64> = conn.query_row(
        "SELECT MAX(position) FROM tasks WHERE column_id = ?1 AND position < ?2",
        params![column_id, before_position],
        |row| row.get(0),
    )?;
    let new_position = match prev_position {
        Some(prev) => (prev + before_position) / 2.0,
        None => before_position / 2.0,
    };
    reposition(conn, id, column_id, new_position)
}

/// Moves a task into `column_id`, appended after every existing task there.
pub fn move_to_end(conn: &Connection, id: i64, column_id: i64) -> Result<()> {
    let position: f64 = conn.query_row(
        "SELECT COALESCE(MAX(position), 0.0) + 1.0 FROM tasks WHERE column_id = ?1",
        params![column_id],
        |row| row.get(0),
    )?;
    reposition(conn, id, column_id, position)
}

fn reposition(conn: &Connection, id: i64, column_id: i64, position: f64) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET column_id = ?1, position = ?2, updated_at = ?3 WHERE id = ?4",
        params![column_id, position, now, id],
    )?;
    Ok(())
}

pub fn complete(conn: &Connection, id: i64) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET completed_at = ?1, updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    Ok(())
}

/// Completes a task, and if it repeats, creates its next occurrence —
/// returning that new task, if one was created.
pub fn complete_and_recur(conn: &Connection, id: i64) -> Result<Option<Task>> {
    complete(conn, id)?;
    let task = get(conn, id)?.ok_or(AppError::NotFound)?;

    let Some(rule) = task.repeat_rule else {
        return Ok(None);
    };

    let due_at = task.due_at.unwrap_or_else(Utc::now);
    let next_task = create(
        conn,
        NewTask {
            title: task.title,
            description: task.description,
            column_id: task.column_id,
            priority: task.priority,
            due_at: Some(rule.next_occurrence(due_at)),
            repeat_rule: Some(rule),
            parent_task_id: task.parent_task_id,
        },
    )?;
    Ok(Some(next_task))
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    Ok(())
}

const SELECT_TASK: &str = "SELECT id, title, description, column_id, position, priority, due_at, completed_at, repeat_rule, parent_task_id, created_at, updated_at FROM tasks {filter}";

fn parse_rfc3339(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .expect("timestamps written by this crate are always valid RFC3339")
        .with_timezone(&Utc)
}

fn row_to_task(row: &Row) -> rusqlite::Result<Task> {
    let due_at: Option<String> = row.get(6)?;
    let completed_at: Option<String> = row.get(7)?;
    let repeat_rule: Option<String> = row.get(8)?;
    let created_at: String = row.get(10)?;
    let updated_at: String = row.get(11)?;

    Ok(Task {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        column_id: row.get(3)?,
        position: row.get(4)?,
        priority: Priority::from_i64(row.get(5)?),
        due_at: due_at.map(|s| parse_rfc3339(&s)),
        completed_at: completed_at.map(|s| parse_rfc3339(&s)),
        repeat_rule: repeat_rule.and_then(|s| RepeatRule::from_db_str(&s)),
        parent_task_id: row.get(9)?,
        created_at: parse_rfc3339(&created_at),
        updated_at: parse_rfc3339(&updated_at),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::NewColumn;
    use crate::storage::{columns, db};

    fn new_task(column_id: i64, title: &str) -> NewTask {
        NewTask {
            title: title.into(),
            description: None,
            column_id,
            priority: Priority::Medium,
            due_at: None,
            repeat_rule: None,
            parent_task_id: None,
        }
    }

    #[test]
    fn create_list_complete_delete_roundtrip() {
        let conn = db::open_in_memory().unwrap();
        let column = columns::create(
            &conn,
            NewColumn {
                name: "Todo".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();

        let task = create(&conn, new_task(column.id, "Write tests")).unwrap();
        assert_eq!(task.position, 1.0);
        assert!(task.completed_at.is_none());

        let second = create(&conn, new_task(column.id, "Second task")).unwrap();
        assert_eq!(second.position, 2.0);

        let listed = list_by_column(&conn, column.id).unwrap();
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].title, "Write tests");

        complete(&conn, task.id).unwrap();
        let fetched = get(&conn, task.id).unwrap().unwrap();
        assert!(fetched.completed_at.is_some());

        delete(&conn, task.id).unwrap();
        assert!(get(&conn, task.id).unwrap().is_none());
    }

    #[test]
    fn subtask_references_its_parent() {
        let conn = db::open_in_memory().unwrap();
        let column = columns::create(
            &conn,
            NewColumn {
                name: "Todo".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();
        let parent = create(&conn, new_task(column.id, "Parent")).unwrap();

        let mut child = new_task(column.id, "Child");
        child.parent_task_id = Some(parent.id);
        let child = create(&conn, child).unwrap();

        assert_eq!(child.parent_task_id, Some(parent.id));
    }

    #[test]
    fn completing_a_recurring_task_creates_its_next_occurrence() {
        let conn = db::open_in_memory().unwrap();
        let column = columns::create(
            &conn,
            NewColumn {
                name: "Todo".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();

        let due = chrono::Utc::now();
        let mut daily = new_task(column.id, "Water plants");
        daily.repeat_rule = Some(crate::domain::RepeatRule::Daily);
        daily.due_at = Some(due);
        let daily = create(&conn, daily).unwrap();

        let next = complete_and_recur(&conn, daily.id).unwrap().unwrap();
        assert_eq!(next.title, "Water plants");
        assert_eq!(next.repeat_rule, Some(crate::domain::RepeatRule::Daily));
        assert!(next.due_at.unwrap() > due);
        assert!(
            get(&conn, daily.id)
                .unwrap()
                .unwrap()
                .completed_at
                .is_some()
        );
    }

    #[test]
    fn completing_a_non_recurring_task_creates_nothing() {
        let conn = db::open_in_memory().unwrap();
        let column = columns::create(
            &conn,
            NewColumn {
                name: "Todo".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();
        let task = create(&conn, new_task(column.id, "One-off")).unwrap();

        let next = complete_and_recur(&conn, task.id).unwrap();
        assert!(next.is_none());
    }

    #[test]
    fn move_before_inserts_between_siblings_without_renumbering_them() {
        let conn = db::open_in_memory().unwrap();
        let todo = columns::create(
            &conn,
            NewColumn {
                name: "Todo".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();
        let doing = columns::create(
            &conn,
            NewColumn {
                name: "Doing".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();

        let a = create(&conn, new_task(todo.id, "A")).unwrap();
        let b = create(&conn, new_task(todo.id, "B")).unwrap();
        assert_eq!((a.position, b.position), (1.0, 2.0));

        // Move a fresh task from Todo into Doing, right before B.
        let moved = create(&conn, new_task(todo.id, "Moving")).unwrap();
        move_before(&conn, moved.id, doing.id, b.position).unwrap();

        let doing_tasks = list_by_column(&conn, doing.id).unwrap();
        assert_eq!(doing_tasks.len(), 1);
        assert_eq!(doing_tasks[0].title, "Moving");

        // B's own position is untouched — no sibling renumbering.
        assert_eq!(get(&conn, b.id).unwrap().unwrap().position, b.position);
    }

    #[test]
    fn move_to_end_appends_after_existing_tasks_in_the_target_column() {
        let conn = db::open_in_memory().unwrap();
        let todo = columns::create(
            &conn,
            NewColumn {
                name: "Todo".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();
        let doing = columns::create(
            &conn,
            NewColumn {
                name: "Doing".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();

        create(&conn, new_task(doing.id, "Existing")).unwrap();
        let moved = create(&conn, new_task(todo.id, "Moving")).unwrap();

        move_to_end(&conn, moved.id, doing.id).unwrap();

        let doing_tasks = list_by_column(&conn, doing.id).unwrap();
        assert_eq!(doing_tasks.len(), 2);
        assert_eq!(doing_tasks.last().unwrap().title, "Moving");
    }

    #[test]
    fn list_all_returns_tasks_across_every_column() {
        let conn = db::open_in_memory().unwrap();
        let todo = columns::create(
            &conn,
            NewColumn {
                name: "Todo".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();
        let doing = columns::create(
            &conn,
            NewColumn {
                name: "Doing".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();
        create(&conn, new_task(todo.id, "In todo")).unwrap();
        create(&conn, new_task(doing.id, "In doing")).unwrap();

        let all = list_all(&conn).unwrap();
        assert_eq!(all.len(), 2);
    }
}
