use rusqlite::{Connection, OptionalExtension, params};

use crate::error::Result;

/// Reads a stored setting, or `None` when it has never been set.
pub fn get(conn: &Connection, key: &str) -> Result<Option<String>> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
    )
    .optional()
    .map_err(Into::into)
}

/// Writes a setting, replacing any previous value.
pub fn set(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db;

    #[test]
    fn unset_key_reads_as_none() {
        let conn = db::open_in_memory().unwrap();
        assert_eq!(get(&conn, "theme").unwrap(), None);
    }

    #[test]
    fn set_then_get_roundtrips() {
        let conn = db::open_in_memory().unwrap();
        set(&conn, "theme", "macos").unwrap();
        assert_eq!(get(&conn, "theme").unwrap(), Some("macos".to_string()));
    }

    #[test]
    fn setting_a_key_twice_replaces_the_value() {
        let conn = db::open_in_memory().unwrap();
        set(&conn, "theme", "light").unwrap();
        set(&conn, "theme", "dark").unwrap();
        assert_eq!(get(&conn, "theme").unwrap(), Some("dark".to_string()));
    }

    #[test]
    fn keys_are_independent() {
        let conn = db::open_in_memory().unwrap();
        set(&conn, "theme", "dark").unwrap();
        set(&conn, "pomodoro.work_seconds", "1500").unwrap();
        assert_eq!(get(&conn, "theme").unwrap(), Some("dark".to_string()));
        assert_eq!(
            get(&conn, "pomodoro.work_seconds").unwrap(),
            Some("1500".to_string())
        );
    }
}
