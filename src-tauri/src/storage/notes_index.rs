use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::domain::NoteMeta;
use crate::error::{AppError, Result};

pub struct NewNoteMeta {
    pub title: String,
    pub file_path: String,
    pub linked_task_id: Option<i64>,
    pub color: String,
    pub rotation_deg: f64,
    pub pos_x: Option<f64>,
    pub pos_y: Option<f64>,
}

pub fn create(conn: &Connection, new_note: NewNoteMeta, body: &str) -> Result<NoteMeta> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO notes
            (title, file_path, linked_task_id, color, rotation_deg, pos_x, pos_y, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
        params![
            new_note.title,
            new_note.file_path,
            new_note.linked_task_id,
            new_note.color,
            new_note.rotation_deg,
            new_note.pos_x,
            new_note.pos_y,
            now,
        ],
    )?;
    let id = conn.last_insert_rowid();
    sync_fts(conn, id, &new_note.title, body)?;
    get(conn, id)?.ok_or(AppError::NotFound)
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<NoteMeta>> {
    conn.query_row(
        &SELECT_NOTE.replace("{filter}", "WHERE id = ?1"),
        params![id],
        row_to_note_meta,
    )
    .optional()
    .map_err(Into::into)
}

pub fn list(conn: &Connection) -> Result<Vec<NoteMeta>> {
    let sql = SELECT_NOTE.replace("{filter}", "ORDER BY id");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_to_note_meta)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Re-indexes a note's search text. Must be called whenever the note's title
/// changes or its markdown body is saved to disk — the FTS table can't see
/// the body on its own, since that text lives in a file, not a column a
/// trigger could observe.
pub fn sync_fts(conn: &Connection, id: i64, title: &str, body: &str) -> Result<()> {
    conn.execute("DELETE FROM notes_fts WHERE rowid = ?1", params![id])?;
    conn.execute(
        "INSERT INTO notes_fts (rowid, title, body) VALUES (?1, ?2, ?3)",
        params![id, title, body],
    )?;
    Ok(())
}

/// Full-text search across note title/body, returning matching note ids
/// ranked by relevance. The query is tokenised and quoted so raw user input
/// can't be read as FTS5 syntax (which errors on stray quotes or operators).
pub fn search(conn: &Connection, query: &str) -> Result<Vec<i64>> {
    let Some(expression) = match_expression(query) else {
        return Ok(Vec::new());
    };

    let mut stmt =
        conn.prepare("SELECT rowid FROM notes_fts WHERE notes_fts MATCH ?1 ORDER BY rank")?;
    let rows = stmt.query_map(params![expression], |row| row.get::<_, i64>(0))?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

pub fn update_file_path(conn: &Connection, id: i64, file_path: &str) -> Result<()> {
    conn.execute(
        "UPDATE notes SET file_path = ?1 WHERE id = ?2",
        params![file_path, id],
    )?;
    Ok(())
}

pub fn update_title(conn: &Connection, id: i64, title: &str) -> Result<()> {
    conn.execute(
        "UPDATE notes SET title = ?1, updated_at = ?2 WHERE id = ?3",
        params![title, Utc::now().to_rfc3339(), id],
    )?;
    Ok(())
}

pub fn update_position(conn: &Connection, id: i64, x: f64, y: f64) -> Result<()> {
    conn.execute(
        "UPDATE notes SET pos_x = ?1, pos_y = ?2 WHERE id = ?3",
        params![x, y, id],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM notes_fts WHERE rowid = ?1", params![id])?;
    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
    Ok(())
}

/// Turns free text into a safe FTS5 MATCH expression: each alphanumeric token
/// becomes a quoted phrase, and tokens without any word characters are dropped.
fn match_expression(query: &str) -> Option<String> {
    let tokens: Vec<String> = query
        .split_whitespace()
        .map(|token| token.replace('"', ""))
        .filter(|token| token.chars().any(char::is_alphanumeric))
        .map(|token| format!("\"{token}\""))
        .collect();

    if tokens.is_empty() {
        None
    } else {
        Some(tokens.join(" "))
    }
}

const SELECT_NOTE: &str = "SELECT id, title, file_path, linked_task_id, color, rotation_deg, pos_x, pos_y, created_at, updated_at FROM notes {filter}";

fn row_to_note_meta(row: &Row) -> rusqlite::Result<NoteMeta> {
    let created_at: String = row.get(8)?;
    let updated_at: String = row.get(9)?;
    Ok(NoteMeta {
        id: row.get(0)?,
        title: row.get(1)?,
        file_path: row.get(2)?,
        linked_task_id: row.get(3)?,
        color: row.get(4)?,
        rotation_deg: row.get(5)?,
        pos_x: row.get(6)?,
        pos_y: row.get(7)?,
        created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
            .unwrap()
            .with_timezone(&Utc),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at)
            .unwrap()
            .with_timezone(&Utc),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db;

    fn new_note(title: &str) -> NewNoteMeta {
        NewNoteMeta {
            title: title.into(),
            file_path: format!("{title}.md"),
            linked_task_id: None,
            color: "#FCEB7E".into(),
            rotation_deg: -3.0,
            pos_x: Some(10.0),
            pos_y: Some(20.0),
        }
    }

    #[test]
    fn create_and_search_note() {
        let conn = db::open_in_memory().unwrap();
        let note = create(
            &conn,
            new_note("Rollout plan"),
            "Talk to infra about staging cutover.",
        )
        .unwrap();

        let found = search(&conn, "staging").unwrap();
        assert_eq!(found, vec![note.id]);

        let missing = search(&conn, "nonexistent").unwrap();
        assert!(missing.is_empty());
    }

    #[test]
    fn delete_removes_note_and_its_fts_row() {
        let conn = db::open_in_memory().unwrap();
        let note = create(&conn, new_note("Temp note"), "disposable body").unwrap();

        delete(&conn, note.id).unwrap();

        assert!(get(&conn, note.id).unwrap().is_none());
        assert!(search(&conn, "disposable").unwrap().is_empty());
    }

    #[test]
    fn search_tolerates_punctuation_and_quotes() {
        let conn = db::open_in_memory().unwrap();
        let note = create(&conn, new_note("Quoting"), "handles \"quoted\" text").unwrap();

        assert_eq!(search(&conn, "\"quoted").unwrap(), vec![note.id]);
        assert_eq!(search(&conn, "handles quoted").unwrap(), vec![note.id]);
        assert!(search(&conn, "   ").unwrap().is_empty());
        assert!(search(&conn, "!!!").unwrap().is_empty());
    }

    #[test]
    fn update_helpers_write_through() {
        let conn = db::open_in_memory().unwrap();
        let note = create(&conn, new_note("Before"), "original").unwrap();

        update_file_path(&conn, note.id, "1-renamed.md").unwrap();
        update_title(&conn, note.id, "After").unwrap();
        update_position(&conn, note.id, 42.0, 84.0).unwrap();

        let updated = get(&conn, note.id).unwrap().unwrap();
        assert_eq!(updated.title, "After");
        assert_eq!(updated.file_path, "1-renamed.md");
        assert_eq!((updated.pos_x, updated.pos_y), (Some(42.0), Some(84.0)));
        assert!(updated.updated_at >= note.updated_at);
    }
}
