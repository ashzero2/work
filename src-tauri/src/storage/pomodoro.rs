use chrono::{Duration, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params};
use serde::{Deserialize, Serialize};

use crate::domain::{PomodoroSession, SessionKind, next_kind};
use crate::error::{AppError, Result};
use crate::storage::settings as settings_store;

/// How long each phase runs, and how many work sessions earn a long break.
/// Held per key in `settings` so the choice survives a restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroSettings {
    pub work_seconds: i64,
    pub break_seconds: i64,
    pub long_break_seconds: i64,
    pub sessions_per_long_break: i64,
}

impl Default for PomodoroSettings {
    fn default() -> Self {
        Self {
            work_seconds: 25 * 60,
            break_seconds: 5 * 60,
            long_break_seconds: 15 * 60,
            sessions_per_long_break: 4,
        }
    }
}

const WORK_KEY: &str = "pomodoro.work_seconds";
const BREAK_KEY: &str = "pomodoro.break_seconds";
const LONG_BREAK_KEY: &str = "pomodoro.long_break_seconds";
const SESSIONS_KEY: &str = "pomodoro.sessions_per_long_break";

const MIN_SECONDS: i64 = 60;
const MAX_SECONDS: i64 = 7200;
const MIN_SESSIONS: i64 = 1;
const MAX_SESSIONS: i64 = 12;

/// The longest a single session may be planned for. The value reaches chrono,
/// which panics on an out-of-range duration — and with `panic = "abort"` that
/// would take the whole app down, so it is clamped at the edge instead.
const MAX_PLANNED_SECONDS: i64 = 24 * 60 * 60;

/// Opens a session. Only one is ever open — the UI resolves any previous one
/// through `reconcile` before starting another.
pub fn start(
    conn: &Connection,
    kind: SessionKind,
    task_id: Option<i64>,
    planned_seconds: i64,
) -> Result<PomodoroSession> {
    let planned_seconds = planned_seconds.clamp(1, MAX_PLANNED_SECONDS);
    conn.execute(
        "INSERT INTO pomodoro_sessions (task_id, kind, planned_seconds, started_at, ended_at, completed)
         VALUES (?1, ?2, ?3, ?4, NULL, 0)",
        params![task_id, kind.as_str(), planned_seconds, Utc::now().to_rfc3339()],
    )?;
    get(conn, conn.last_insert_rowid())?.ok_or(AppError::NotFound)
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<PomodoroSession>> {
    conn.query_row(
        &SELECT_SESSION.replace("{filter}", "WHERE id = ?1"),
        params![id],
        row_to_session,
    )
    .optional()
    .map_err(Into::into)
}

/// Ends a session, recording whether it ran its full course.
pub fn finish(conn: &Connection, id: i64, completed: bool) -> Result<PomodoroSession> {
    conn.execute(
        "UPDATE pomodoro_sessions SET ended_at = ?1, completed = ?2 WHERE id = ?3",
        params![Utc::now().to_rfc3339(), completed, id],
    )?;
    get(conn, id)?.ok_or(AppError::NotFound)
}

pub fn open(conn: &Connection) -> Result<Option<PomodoroSession>> {
    conn.query_row(
        &SELECT_SESSION.replace(
            "{filter}",
            "WHERE ended_at IS NULL ORDER BY id DESC LIMIT 1",
        ),
        [],
        row_to_session,
    )
    .optional()
    .map_err(Into::into)
}

/// Run at startup. A session still inside its planned window is handed back so
/// the UI can resume it; one that has already run out is closed as abandoned, so
/// quitting the app can never leave a timer that never ends.
pub fn reconcile(conn: &Connection) -> Result<Option<PomodoroSession>> {
    let Some(session) = open(conn)? else {
        return Ok(None);
    };

    let planned_end = session.started_at + Duration::seconds(session.planned_seconds);
    if planned_end > Utc::now() {
        Ok(Some(session))
    } else {
        finish(conn, session.id, false)?;
        Ok(None)
    }
}

/// Newest first — the queryable history the phase calls for, with no report UI
/// built on top of it yet.
pub fn recent(conn: &Connection, limit: i64) -> Result<Vec<PomodoroSession>> {
    let sql = SELECT_SESSION.replace("{filter}", "ORDER BY id DESC LIMIT ?1");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![limit.max(1)], row_to_session)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Work sessions completed since the last completed long break. Derived from the
/// table rather than tracked in the UI, so the cycle survives a restart.
pub fn work_sessions_since_long_break(conn: &Connection) -> Result<i64> {
    conn.query_row(
        "SELECT count(*) FROM pomodoro_sessions
         WHERE kind = 'work' AND completed = 1
           AND id > COALESCE((SELECT MAX(id) FROM pomodoro_sessions
                              WHERE kind = 'long_break' AND completed = 1), 0)",
        [],
        |row| row.get(0),
    )
    .map_err(Into::into)
}

/// The phase that follows the most recent session, or work when there is none.
pub fn next_phase(conn: &Connection) -> Result<SessionKind> {
    let Some(current) = latest_kind(conn)? else {
        return Ok(SessionKind::Work);
    };
    let completed = work_sessions_since_long_break(conn)?;
    Ok(next_kind(
        current,
        completed,
        settings(conn)?.sessions_per_long_break,
    ))
}

pub fn settings(conn: &Connection) -> Result<PomodoroSettings> {
    let defaults = PomodoroSettings::default();
    Ok(PomodoroSettings {
        work_seconds: read_number(
            conn,
            WORK_KEY,
            defaults.work_seconds,
            MIN_SECONDS,
            MAX_SECONDS,
        )?,
        break_seconds: read_number(
            conn,
            BREAK_KEY,
            defaults.break_seconds,
            MIN_SECONDS,
            MAX_SECONDS,
        )?,
        long_break_seconds: read_number(
            conn,
            LONG_BREAK_KEY,
            defaults.long_break_seconds,
            MIN_SECONDS,
            MAX_SECONDS,
        )?,
        sessions_per_long_break: read_number(
            conn,
            SESSIONS_KEY,
            defaults.sessions_per_long_break,
            MIN_SESSIONS,
            MAX_SESSIONS,
        )?,
    })
}

/// Stores the durations, clamped, and returns what was actually written.
pub fn save_settings(conn: &Connection, settings: &PomodoroSettings) -> Result<PomodoroSettings> {
    let clamped = PomodoroSettings {
        work_seconds: settings.work_seconds.clamp(MIN_SECONDS, MAX_SECONDS),
        break_seconds: settings.break_seconds.clamp(MIN_SECONDS, MAX_SECONDS),
        long_break_seconds: settings.long_break_seconds.clamp(MIN_SECONDS, MAX_SECONDS),
        sessions_per_long_break: settings
            .sessions_per_long_break
            .clamp(MIN_SESSIONS, MAX_SESSIONS),
    };

    settings_store::set(conn, WORK_KEY, &clamped.work_seconds.to_string())?;
    settings_store::set(conn, BREAK_KEY, &clamped.break_seconds.to_string())?;
    settings_store::set(
        conn,
        LONG_BREAK_KEY,
        &clamped.long_break_seconds.to_string(),
    )?;
    settings_store::set(
        conn,
        SESSIONS_KEY,
        &clamped.sessions_per_long_break.to_string(),
    )?;

    Ok(clamped)
}

fn latest_kind(conn: &Connection) -> Result<Option<SessionKind>> {
    let value: Option<String> = conn
        .query_row(
            "SELECT kind FROM pomodoro_sessions ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .optional()?;
    Ok(value.and_then(|value| SessionKind::from_db_str(&value)))
}

/// Falls back to the default and clamps anything a corrupt or hand-edited row
/// might hold, so the timer can't be handed a nonsensical duration.
fn read_number(conn: &Connection, key: &str, default: i64, min: i64, max: i64) -> Result<i64> {
    Ok(settings_store::get(conn, key)?
        .and_then(|value| value.parse::<i64>().ok())
        .map(|value| value.clamp(min, max))
        .unwrap_or(default))
}

const SELECT_SESSION: &str = "SELECT id, task_id, kind, planned_seconds, started_at, ended_at, completed FROM pomodoro_sessions {filter}";

fn row_to_session(row: &Row) -> rusqlite::Result<PomodoroSession> {
    let kind: String = row.get(2)?;
    let started_at: String = row.get(4)?;
    let ended_at: Option<String> = row.get(5)?;
    Ok(PomodoroSession {
        id: row.get(0)?,
        task_id: row.get(1)?,
        kind: SessionKind::from_db_str(&kind).unwrap_or(SessionKind::Work),
        planned_seconds: row.get(3)?,
        started_at: parse_rfc3339(&started_at)?,
        ended_at: ended_at.map(|value| parse_rfc3339(&value)).transpose()?,
        completed: row.get(6)?,
    })
}

/// A stored timestamp that no longer parses is reported as a row error rather
/// than panicking: a hand-edited or partially-written row must not be able to
/// abort the app.
fn parse_rfc3339(value: &str) -> rusqlite::Result<chrono::DateTime<Utc>> {
    chrono::DateTime::parse_from_rfc3339(value)
        .map(|parsed| parsed.with_timezone(&Utc))
        .map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{NewColumn, NewTask, Priority};
    use crate::storage::{columns, db, tasks};

    fn conn() -> Connection {
        db::open_in_memory().unwrap()
    }

    fn complete(conn: &Connection, kind: SessionKind, seconds: i64) {
        let session = start(conn, kind, None, seconds).unwrap();
        finish(conn, session.id, true).unwrap();
    }

    #[test]
    fn start_clamps_an_absurd_planned_length() {
        let conn = conn();
        assert_eq!(
            start(&conn, SessionKind::Work, None, i64::MAX)
                .unwrap()
                .planned_seconds,
            MAX_PLANNED_SECONDS
        );
        assert_eq!(
            start(&conn, SessionKind::Break, None, -10)
                .unwrap()
                .planned_seconds,
            1
        );
    }

    #[test]
    fn start_leaves_a_session_open() {
        let conn = conn();
        let session = start(&conn, SessionKind::Work, None, 1500).unwrap();

        assert_eq!(session.kind, SessionKind::Work);
        assert_eq!(session.planned_seconds, 1500);
        assert_eq!(session.task_id, None);
        assert!(session.ended_at.is_none());
        assert!(!session.completed);
        assert_eq!(open(&conn).unwrap().map(|found| found.id), Some(session.id));
    }

    #[test]
    fn start_can_attach_a_task() {
        let conn = conn();
        let column = columns::create(
            &conn,
            NewColumn {
                name: "Todo".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();
        let task = tasks::create(
            &conn,
            NewTask {
                title: "Write the thing".into(),
                description: None,
                column_id: column.id,
                priority: Priority::None,
                due_at: None,
                repeat_rule: None,
                parent_task_id: None,
            },
        )
        .unwrap();

        let session = start(&conn, SessionKind::Work, Some(task.id), 1500).unwrap();
        assert_eq!(session.task_id, Some(task.id));
    }

    #[test]
    fn finish_stamps_the_end_and_the_outcome() {
        let conn = conn();
        let session = start(&conn, SessionKind::Break, None, 300).unwrap();

        let finished = finish(&conn, session.id, true).unwrap();

        assert!(finished.ended_at.is_some());
        assert!(finished.completed);
        assert!(open(&conn).unwrap().is_none());
    }

    #[test]
    fn reconcile_hands_back_a_session_that_is_still_running() {
        let conn = conn();
        let session = start(&conn, SessionKind::Work, None, 1500).unwrap();

        assert_eq!(
            reconcile(&conn).unwrap().map(|found| found.id),
            Some(session.id)
        );
    }

    #[test]
    fn reconcile_closes_a_session_whose_time_ran_out() {
        let conn = conn();
        let stale = Utc::now() - Duration::hours(2);
        conn.execute(
            "INSERT INTO pomodoro_sessions (kind, planned_seconds, started_at, completed)
             VALUES ('work', 1500, ?1, 0)",
            params![stale.to_rfc3339()],
        )
        .unwrap();

        assert!(reconcile(&conn).unwrap().is_none());

        let listed = recent(&conn, 1).unwrap();
        assert!(listed[0].ended_at.is_some());
        assert!(
            !listed[0].completed,
            "an abandoned session is not a success"
        );
    }

    #[test]
    fn reconcile_is_a_no_op_with_nothing_open() {
        let conn = conn();
        assert!(reconcile(&conn).unwrap().is_none());
    }

    #[test]
    fn the_cycle_counts_only_completed_work_and_resets_on_a_long_break() {
        let conn = conn();
        for _ in 0..3 {
            complete(&conn, SessionKind::Work, 60);
        }
        assert_eq!(work_sessions_since_long_break(&conn).unwrap(), 3);

        complete(&conn, SessionKind::LongBreak, 900);
        assert_eq!(work_sessions_since_long_break(&conn).unwrap(), 0);

        complete(&conn, SessionKind::Work, 60);
        assert_eq!(work_sessions_since_long_break(&conn).unwrap(), 1);
    }

    #[test]
    fn an_abandoned_work_session_does_not_count_towards_the_cycle() {
        let conn = conn();
        let session = start(&conn, SessionKind::Work, None, 60).unwrap();
        finish(&conn, session.id, false).unwrap();

        assert_eq!(work_sessions_since_long_break(&conn).unwrap(), 0);
    }

    #[test]
    fn next_phase_starts_with_work() {
        let conn = conn();
        assert_eq!(next_phase(&conn).unwrap(), SessionKind::Work);
    }

    #[test]
    fn next_phase_follows_the_cycle() {
        let conn = conn();

        for completed in 1..=3 {
            complete(&conn, SessionKind::Work, 60);
            assert_eq!(
                next_phase(&conn).unwrap(),
                SessionKind::Break,
                "after {completed} work sessions"
            );
        }

        complete(&conn, SessionKind::Work, 60);
        assert_eq!(next_phase(&conn).unwrap(), SessionKind::LongBreak);

        complete(&conn, SessionKind::LongBreak, 900);
        assert_eq!(next_phase(&conn).unwrap(), SessionKind::Work);
    }

    #[test]
    fn recent_lists_newest_first() {
        let conn = conn();
        let first = start(&conn, SessionKind::Work, None, 60).unwrap();
        let second = start(&conn, SessionKind::Break, None, 60).unwrap();

        let listed = recent(&conn, 10).unwrap();

        assert_eq!(
            listed.iter().map(|session| session.id).collect::<Vec<_>>(),
            vec![second.id, first.id]
        );
    }

    #[test]
    fn settings_default_when_nothing_has_been_stored() {
        let conn = conn();
        assert_eq!(settings(&conn).unwrap(), PomodoroSettings::default());
    }

    #[test]
    fn settings_roundtrip_and_clamp_out_of_range_values() {
        let conn = conn();
        let wanted = PomodoroSettings {
            work_seconds: 3000,
            break_seconds: 600,
            long_break_seconds: 1200,
            sessions_per_long_break: 3,
        };
        assert_eq!(save_settings(&conn, &wanted).unwrap(), wanted);
        assert_eq!(settings(&conn).unwrap(), wanted);

        let absurd = PomodoroSettings {
            work_seconds: 1,
            break_seconds: 999_999,
            long_break_seconds: 0,
            sessions_per_long_break: 99,
        };
        let clamped = save_settings(&conn, &absurd).unwrap();

        assert_eq!(clamped.work_seconds, MIN_SECONDS);
        assert_eq!(clamped.break_seconds, MAX_SECONDS);
        assert_eq!(clamped.long_break_seconds, MIN_SECONDS);
        assert_eq!(clamped.sessions_per_long_break, MAX_SESSIONS);
    }
}
