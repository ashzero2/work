use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::domain::{EntityKind, Tag};
use crate::error::{AppError, Result};

pub fn list(conn: &Connection) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare("SELECT id, name, color FROM tags ORDER BY name COLLATE NOCASE")?;
    let rows = stmt.query_map([], row_to_tag)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<Tag>> {
    conn.query_row(
        "SELECT id, name, color FROM tags WHERE id = ?1",
        params![id],
        row_to_tag,
    )
    .optional()
    .map_err(Into::into)
}

/// Returns the tag with this name, creating it if needed. Matching is
/// case-insensitive so `Work` and `work` stay a single tag.
pub fn get_or_create(conn: &Connection, name: &str) -> Result<Tag> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(AppError::InvalidInput("tag name cannot be empty".into()));
    }

    let existing = conn
        .query_row(
            "SELECT id, name, color FROM tags WHERE name = ?1 COLLATE NOCASE",
            params![trimmed],
            row_to_tag,
        )
        .optional()?;
    if let Some(tag) = existing {
        return Ok(tag);
    }

    conn.execute("INSERT INTO tags (name) VALUES (?1)", params![trimmed])?;
    get(conn, conn.last_insert_rowid())?.ok_or(AppError::NotFound)
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM taggables WHERE tag_id = ?1", params![id])?;
    conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn attach(conn: &Connection, tag_id: i64, kind: EntityKind, entity_id: i64) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO taggables (tag_id, entity_type, entity_id) VALUES (?1, ?2, ?3)",
        params![tag_id, kind.as_str(), entity_id],
    )?;
    Ok(())
}

pub fn detach(conn: &Connection, tag_id: i64, kind: EntityKind, entity_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM taggables WHERE tag_id = ?1 AND entity_type = ?2 AND entity_id = ?3",
        params![tag_id, kind.as_str(), entity_id],
    )?;
    Ok(())
}

pub fn names_for_entity(
    conn: &Connection,
    kind: EntityKind,
    entity_id: i64,
) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT t.name
         FROM tags t
         JOIN taggables g ON g.tag_id = t.id
         WHERE g.entity_type = ?1 AND g.entity_id = ?2
         ORDER BY t.name COLLATE NOCASE",
    )?;
    let rows = stmt.query_map(params![kind.as_str(), entity_id], |row| {
        row.get::<_, String>(0)
    })?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Replaces an entity's whole tag set, creating any tags that don't exist yet.
pub fn set_for_entity(
    conn: &Connection,
    kind: EntityKind,
    entity_id: i64,
    names: &[String],
) -> Result<Vec<Tag>> {
    conn.execute(
        "DELETE FROM taggables WHERE entity_type = ?1 AND entity_id = ?2",
        params![kind.as_str(), entity_id],
    )?;

    let mut applied: Vec<Tag> = Vec::new();
    for name in names {
        let tag = get_or_create(conn, name)?;
        if applied.iter().any(|existing| existing.id == tag.id) {
            continue;
        }
        attach(conn, tag.id, kind, entity_id)?;
        applied.push(tag);
    }
    Ok(applied)
}

/// Tags attached to at least one entity of this kind, with their usage count.
pub fn list_with_counts(conn: &Connection, kind: EntityKind) -> Result<Vec<(Tag, i64)>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, t.color, COUNT(g.entity_id)
         FROM tags t
         LEFT JOIN taggables g ON g.tag_id = t.id AND g.entity_type = ?1
         GROUP BY t.id
         HAVING COUNT(g.entity_id) > 0
         ORDER BY t.name COLLATE NOCASE",
    )?;
    let rows = stmt.query_map(params![kind.as_str()], |row| {
        Ok((row_to_tag(row)?, row.get::<_, i64>(3)?))
    })?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

fn row_to_tag(row: &Row) -> rusqlite::Result<Tag> {
    Ok(Tag {
        id: row.get(0)?,
        name: row.get(1)?,
        color: row.get(2)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db;

    #[test]
    fn get_or_create_is_idempotent_and_case_insensitive() {
        let conn = db::open_in_memory().unwrap();
        let first = get_or_create(&conn, "Work").unwrap();
        let second = get_or_create(&conn, "  work ").unwrap();

        assert_eq!(first.id, second.id);
        assert_eq!(first.name, "Work");
        assert_eq!(list(&conn).unwrap().len(), 1);
    }

    #[test]
    fn empty_tag_name_is_rejected() {
        let conn = db::open_in_memory().unwrap();
        assert!(matches!(
            get_or_create(&conn, "   ").unwrap_err(),
            AppError::InvalidInput(_)
        ));
    }

    #[test]
    fn set_for_entity_replaces_the_previous_set() {
        let conn = db::open_in_memory().unwrap();
        set_for_entity(&conn, EntityKind::Note, 1, &["work".into(), "q3".into()]).unwrap();
        assert_eq!(
            names_for_entity(&conn, EntityKind::Note, 1).unwrap(),
            vec!["q3", "work"]
        );

        set_for_entity(&conn, EntityKind::Note, 1, &["personal".into()]).unwrap();
        assert_eq!(
            names_for_entity(&conn, EntityKind::Note, 1).unwrap(),
            vec!["personal"]
        );
    }

    #[test]
    fn set_for_entity_ignores_duplicate_names() {
        let conn = db::open_in_memory().unwrap();
        let applied =
            set_for_entity(&conn, EntityKind::Note, 1, &["work".into(), "work".into()]).unwrap();
        assert_eq!(applied.len(), 1);
    }

    #[test]
    fn counts_only_include_tags_in_use_for_that_kind() {
        let conn = db::open_in_memory().unwrap();
        set_for_entity(&conn, EntityKind::Note, 1, &["work".into(), "q3".into()]).unwrap();
        set_for_entity(&conn, EntityKind::Note, 2, &["work".into()]).unwrap();
        set_for_entity(&conn, EntityKind::Task, 1, &["urgent".into()]).unwrap();

        let note_counts = list_with_counts(&conn, EntityKind::Note).unwrap();
        let names: Vec<(&str, i64)> = note_counts
            .iter()
            .map(|(tag, count)| (tag.name.as_str(), *count))
            .collect();
        assert_eq!(names, vec![("q3", 1), ("work", 2)]);
    }

    #[test]
    fn deleting_a_tag_clears_its_links() {
        let conn = db::open_in_memory().unwrap();
        let tag = get_or_create(&conn, "work").unwrap();
        attach(&conn, tag.id, EntityKind::Note, 1).unwrap();

        delete(&conn, tag.id).unwrap();

        assert!(
            names_for_entity(&conn, EntityKind::Note, 1)
                .unwrap()
                .is_empty()
        );
        assert!(
            list_with_counts(&conn, EntityKind::Note)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn attach_is_idempotent_and_detach_removes_the_link() {
        let conn = db::open_in_memory().unwrap();
        let tag = get_or_create(&conn, "work").unwrap();
        attach(&conn, tag.id, EntityKind::Note, 1).unwrap();
        attach(&conn, tag.id, EntityKind::Note, 1).unwrap();
        assert_eq!(
            names_for_entity(&conn, EntityKind::Note, 1).unwrap(),
            vec!["work"]
        );

        detach(&conn, tag.id, EntityKind::Note, 1).unwrap();
        assert!(
            names_for_entity(&conn, EntityKind::Note, 1)
                .unwrap()
                .is_empty()
        );
    }
}
