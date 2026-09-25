use std::path::Path;

use chrono::Utc;
use rusqlite::Connection;

use crate::domain::{EntityKind, NoteFrontMatter, NoteMeta};
use crate::error::{AppError, Result};
use crate::storage::{notes_fs, notes_index, tags};

const PALETTE: [&str; 6] = [
    "#FCEB7E", "#FFD8A8", "#C7F0DB", "#CDE4FF", "#F3D1FF", "#FFD1DC",
];
const COLUMN_WIDTH: f64 = 240.0;
const ROW_HEIGHT: f64 = 176.0;
const CANVAS_COLUMNS: usize = 4;

pub struct NewNote {
    pub title: String,
}

/// Creates a note: the index row first (to obtain the id that names the
/// file), then the `.md` itself, then the row's file path — all in one
/// transaction so a failed write can't leave a row pointing at nothing.
pub fn create(conn: &Connection, notes_dir: &Path, new_note: NewNote) -> Result<NoteMeta> {
    let title = new_note.title.trim();
    if title.is_empty() {
        return Err(AppError::InvalidInput("note title cannot be empty".into()));
    }

    let seed = Utc::now().timestamp_nanos_opt().unwrap_or_default();
    let index = notes_index::list(conn)?.len();

    let tx = conn.unchecked_transaction()?;
    let meta = notes_index::create(
        &tx,
        notes_index::NewNoteMeta {
            title: title.to_string(),
            file_path: String::new(),
            linked_task_id: None,
            color: PALETTE[(seed.unsigned_abs() as usize) % PALETTE.len()].to_string(),
            rotation_deg: rotation(seed),
            pos_x: Some(scatter(index, seed, 0)),
            pos_y: Some(scatter(index, seed, 1)),
        },
        "",
    )?;

    let file_name = format!("{}-{}.md", meta.id, slug(title));
    let front_matter = NoteFrontMatter {
        id: meta.id,
        title: title.to_string(),
        tags: Vec::new(),
        linked_task_id: None,
        created_at: meta.created_at,
        updated_at: meta.updated_at,
    };
    notes_fs::write(
        &notes_fs::note_path(notes_dir, &file_name),
        &front_matter,
        "",
    )?;
    notes_index::update_file_path(&tx, meta.id, &file_name)?;
    tx.commit()?;

    notes_index::get(conn, meta.id)?.ok_or(AppError::NotFound)
}

/// The note's markdown body, read from disk on demand — the one part of a
/// note that isn't held in the index.
pub fn body(conn: &Connection, notes_dir: &Path, id: i64) -> Result<String> {
    let meta = notes_index::get(conn, id)?.ok_or(AppError::NotFound)?;
    let (_, body) = notes_fs::read(&notes_fs::note_path(notes_dir, &meta.file_path))?;
    Ok(body)
}

/// Rewrites the note's file and index together, so the markdown frontmatter
/// and the queryable row can't drift apart.
pub fn save(
    conn: &Connection,
    notes_dir: &Path,
    id: i64,
    title: &str,
    body: &str,
    tag_names: &[String],
) -> Result<NoteMeta> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(AppError::InvalidInput("note title cannot be empty".into()));
    }

    let meta = notes_index::get(conn, id)?.ok_or(AppError::NotFound)?;
    tags::set_for_entity(conn, EntityKind::Note, id, tag_names)?;
    let names = tags::names_for_entity(conn, EntityKind::Note, id)?;

    let front_matter = NoteFrontMatter {
        id,
        title: trimmed.to_string(),
        tags: names,
        linked_task_id: meta.linked_task_id,
        created_at: meta.created_at,
        updated_at: Utc::now(),
    };
    notes_fs::write(
        &notes_fs::note_path(notes_dir, &meta.file_path),
        &front_matter,
        body,
    )?;

    notes_index::update_title(conn, id, trimmed)?;
    notes_index::sync_fts(conn, id, trimmed, body)?;

    notes_index::get(conn, id)?.ok_or(AppError::NotFound)
}

pub fn delete(conn: &Connection, notes_dir: &Path, id: i64) -> Result<()> {
    let meta = notes_index::get(conn, id)?.ok_or(AppError::NotFound)?;

    tags::set_for_entity(conn, EntityKind::Note, id, &[])?;
    notes_index::delete(conn, id)?;
    if !meta.file_path.is_empty() {
        notes_fs::remove(&notes_fs::note_path(notes_dir, &meta.file_path))?;
    }
    Ok(())
}

fn rotation(seed: i64) -> f64 {
    (seed.unsigned_abs() % 800) as f64 / 100.0 - 4.0
}

fn scatter(index: usize, seed: i64, axis: u8) -> f64 {
    let column = (index % CANVAS_COLUMNS) as f64 * COLUMN_WIDTH;
    let row = (index / CANVAS_COLUMNS) as f64 * ROW_HEIGHT;
    let jitter = (seed.rotate_left(u32::from(axis) * 7 + 3).unsigned_abs() % 28) as f64;
    match axis {
        0 => 32.0 + column + jitter,
        _ => 32.0 + row + jitter,
    }
}

fn slug(title: &str) -> String {
    let mut slug = String::new();
    for ch in title.chars() {
        if ch.is_alphanumeric() {
            slug.extend(ch.to_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }

    let trimmed = slug.trim_matches('-');
    let capped: String = trimmed.chars().take(48).collect();
    let capped = capped.trim_matches('-');
    if capped.is_empty() {
        "note".to_string()
    } else {
        capped.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db;
    use tempfile::tempdir;

    fn create_note(conn: &Connection, dir: &Path, title: &str) -> NoteMeta {
        create(
            conn,
            dir,
            NewNote {
                title: title.into(),
            },
        )
        .unwrap()
    }

    #[test]
    fn create_writes_a_file_that_matches_the_row() {
        let conn = db::open_in_memory().unwrap();
        let dir = tempdir().unwrap();
        let note = create_note(&conn, dir.path(), "Rollout plan");

        assert!(note.file_path.starts_with(&format!("{}-", note.id)));
        assert!(note.file_path.ends_with(".md"));
        assert!(note.pos_x.is_some() && note.pos_y.is_some());

        let (front_matter, body) = notes_fs::read(&dir.path().join(&note.file_path)).unwrap();
        assert_eq!(front_matter.id, note.id);
        assert_eq!(front_matter.title, "Rollout plan");
        assert!(body.trim().is_empty());
    }

    #[test]
    fn save_roundtrips_body_and_tags_and_updates_search() {
        let conn = db::open_in_memory().unwrap();
        let dir = tempdir().unwrap();
        let note = create_note(&conn, dir.path(), "Kickoff");

        let saved = save(
            &conn,
            dir.path(),
            note.id,
            "Kickoff notes",
            "talk to infra about staging",
            &["work".into(), "q3".into()],
        )
        .unwrap();

        assert_eq!(saved.title, "Kickoff notes");
        assert_eq!(
            notes_index::search(&conn, "staging").unwrap(),
            vec![note.id]
        );
        assert_eq!(
            tags::names_for_entity(&conn, EntityKind::Note, note.id).unwrap(),
            vec!["q3", "work"]
        );

        let (front_matter, body) = notes_fs::read(&dir.path().join(&note.file_path)).unwrap();
        assert_eq!(front_matter.title, "Kickoff notes");
        assert_eq!(front_matter.tags, vec!["q3", "work"]);
        assert_eq!(body.trim(), "talk to infra about staging");
    }

    #[test]
    fn body_reads_back_what_save_wrote() {
        let conn = db::open_in_memory().unwrap();
        let dir = tempdir().unwrap();
        let note = create_note(&conn, dir.path(), "Doc");

        save(&conn, dir.path(), note.id, "Doc", "line one\nline two", &[]).unwrap();

        assert_eq!(
            body(&conn, dir.path(), note.id).unwrap().trim(),
            "line one\nline two"
        );
    }

    #[test]
    fn delete_removes_the_row_the_file_and_the_links() {
        let conn = db::open_in_memory().unwrap();
        let dir = tempdir().unwrap();
        let note = create_note(&conn, dir.path(), "Throwaway");
        save(
            &conn,
            dir.path(),
            note.id,
            "Throwaway",
            "body text",
            &["temp".into()],
        )
        .unwrap();
        let path = dir.path().join(&note.file_path);
        assert!(path.exists());

        delete(&conn, dir.path(), note.id).unwrap();

        assert!(!path.exists());
        assert!(notes_index::get(&conn, note.id).unwrap().is_none());
        assert!(notes_index::search(&conn, "body").unwrap().is_empty());
        assert!(
            tags::names_for_entity(&conn, EntityKind::Note, note.id)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn new_notes_do_not_land_on_the_same_spot() {
        let conn = db::open_in_memory().unwrap();
        let dir = tempdir().unwrap();
        let first = create_note(&conn, dir.path(), "One");
        let second = create_note(&conn, dir.path(), "Two");

        assert_ne!((first.pos_x, first.pos_y), (second.pos_x, second.pos_y));
    }

    #[test]
    fn empty_title_is_rejected() {
        let conn = db::open_in_memory().unwrap();
        let dir = tempdir().unwrap();

        let error = create(
            &conn,
            dir.path(),
            NewNote {
                title: "   ".into(),
            },
        )
        .unwrap_err();
        assert!(matches!(error, AppError::InvalidInput(_)));
    }

    #[test]
    fn slug_falls_back_and_strips_punctuation() {
        assert_eq!(slug("Meeting — Q3 / kickoff!"), "meeting-q3-kickoff");
        assert_eq!(slug("!!!"), "note");
    }
}
