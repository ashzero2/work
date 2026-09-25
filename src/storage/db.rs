use std::path::Path;

use rusqlite::Connection;

use crate::error::Result;
use crate::storage::migrations::MIGRATIONS;
use crate::storage::paths;

/// Opens (creating if needed) the app's real SQLite database and brings it
/// up to the latest migration.
pub fn open() -> Result<Connection> {
    std::fs::create_dir_all(paths::data_dir())?;
    open_at(&paths::db_path())
}

/// Opens a database at an explicit path — used by callers (e.g. tests) that
/// need to avoid touching the real `~/work-dashboard` directory.
pub fn open_at(path: &Path) -> Result<Connection> {
    let mut conn = Connection::open(path)?;
    conn.pragma_update(None, "foreign_keys", true)?;
    MIGRATIONS.to_latest(&mut conn)?;
    Ok(conn)
}

/// An in-memory database, migrated to the latest schema — the default for tests.
pub fn open_in_memory() -> Result<Connection> {
    let mut conn = Connection::open_in_memory()?;
    conn.pragma_update(None, "foreign_keys", true)?;
    MIGRATIONS.to_latest(&mut conn)?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_apply_cleanly_on_an_empty_db() {
        let conn = open_in_memory().expect("open in-memory db");
        let table_count: i64 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type IN ('table', 'virtual table' )",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(
            table_count >= 7,
            "expected at least 7 tables, found {table_count}"
        );
    }
}
