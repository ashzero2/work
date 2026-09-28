use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::domain::{Reminder, ReminderKind, ReminderStatus};
use crate::error::{AppError, Result};
use crate::storage::settings;

pub struct NewReminder {
    pub kind: ReminderKind,
    pub task_id: Option<i64>,
    pub session_id: Option<i64>,
    pub trigger_at: DateTime<Utc>,
}

/// How far a Snooze action pushes a reminder out. Shared so the UI's own Snooze
/// button and the notification's button can't drift apart.
pub const SNOOZE_MINUTES: i64 = 10;

pub fn create(conn: &Connection, new_reminder: NewReminder) -> Result<Reminder> {
    conn.execute(
        "INSERT INTO reminders (kind, task_id, session_id, trigger_at, status)
         VALUES (?1, ?2, ?3, ?4, 'pending')",
        params![
            new_reminder.kind.as_str(),
            new_reminder.task_id,
            new_reminder.session_id,
            new_reminder.trigger_at.to_rfc3339(),
        ],
    )?;
    get(conn, conn.last_insert_rowid())?.ok_or(AppError::NotFound)
}

/// Creates the reminder that announces a Pomodoro session's end.
pub fn create_for_session(
    conn: &Connection,
    session_id: i64,
    kind: ReminderKind,
    trigger_at: DateTime<Utc>,
) -> Result<Reminder> {
    create(
        conn,
        NewReminder {
            kind,
            task_id: None,
            session_id: Some(session_id),
            trigger_at,
        },
    )
}

/// Withdraws a session's reminder, however that session ended. Marked rather
/// than deleted so the next sync still sees the row and can cancel it with the
/// system: a deleted row is invisible to sync, which would leave the OS holding
/// a notification nothing can reach any more.
pub fn withdraw_for_session(conn: &Connection, session_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE reminders SET status = 'dismissed'
         WHERE session_id = ?1 AND status IN ('pending', 'snoozed')",
        params![session_id],
    )?;
    Ok(())
}

pub fn get(conn: &Connection, id: i64) -> Result<Option<Reminder>> {
    conn.query_row(
        &SELECT_REMINDER.replace("{filter}", "WHERE id = ?1"),
        params![id],
        row_to_reminder,
    )
    .optional()
    .map_err(Into::into)
}

/// Outstanding reminders first (soonest first), then the ones already dealt
/// with — so the list reads as a to-do rather than a log.
pub fn list(conn: &Connection) -> Result<Vec<Reminder>> {
    let sql = SELECT_REMINDER.replace(
        "{filter}",
        "ORDER BY CASE status WHEN 'pending' THEN 0 WHEN 'snoozed' THEN 1 ELSE 2 END,
                  trigger_at",
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_to_reminder)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// The reminder a system notification belongs to, resolved from the identifier
/// it was registered under.
pub fn find_by_tag(conn: &Connection, tag: &str) -> Result<Option<Reminder>> {
    conn.query_row(
        &SELECT_REMINDER.replace("{filter}", "WHERE system_notification_tag = ?1"),
        params![tag],
        row_to_reminder,
    )
    .optional()
    .map_err(Into::into)
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM reminders WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn reschedule(conn: &Connection, id: i64, trigger_at: DateTime<Utc>) -> Result<Reminder> {
    conn.execute(
        "UPDATE reminders SET trigger_at = ?1, status = 'pending', snoozed_until = NULL
         WHERE id = ?2",
        params![trigger_at.to_rfc3339(), id],
    )?;
    get(conn, id)?.ok_or(AppError::NotFound)
}

/// Records the outcome the OS reported: the user opened it, or dismissed it.
pub fn mark(conn: &Connection, id: i64, status: ReminderStatus) -> Result<Reminder> {
    conn.execute(
        "UPDATE reminders SET status = ?1 WHERE id = ?2",
        params![status.as_str(), id],
    )?;
    get(conn, id)?.ok_or(AppError::NotFound)
}

/// Re-arms a reminder for later, which is what a Snooze action means: the row
/// stays snoozed until the new time, and is registered with the system again.
pub fn snooze(conn: &Connection, id: i64, until: DateTime<Utc>) -> Result<Reminder> {
    conn.execute(
        "UPDATE reminders SET status = 'snoozed', snoozed_until = ?1, trigger_at = ?1
         WHERE id = ?2",
        params![until.to_rfc3339(), id],
    )?;
    get(conn, id)?.ok_or(AppError::NotFound)
}

/// The identifier the reminder is registered under with the system, so it can be
/// cancelled or traced back from an action callback.
pub fn set_system_tag(conn: &Connection, id: i64, tag: Option<&str>) -> Result<()> {
    conn.execute(
        "UPDATE reminders SET system_notification_tag = ?1 WHERE id = ?2",
        params![tag, id],
    )?;
    Ok(())
}

/// Everything that should currently be registered with the OS.
pub fn pending(conn: &Connection) -> Result<Vec<Reminder>> {
    let sql = SELECT_REMINDER.replace(
        "{filter}",
        "WHERE status IN ('pending', 'snoozed') ORDER BY trigger_at",
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_to_reminder)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Pending reminders whose time has arrived — what the app reconciles against on
/// launch, since the OS may have delivered them while it wasn't running.
pub fn due(conn: &Connection, at: DateTime<Utc>) -> Result<Vec<Reminder>> {
    let sql = SELECT_REMINDER.replace(
        "{filter}",
        "WHERE status = 'pending' AND trigger_at <= ?1 ORDER BY trigger_at",
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![at.to_rfc3339()], row_to_reminder)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Whether notifications are wanted for this kind. Unset means on, so a fresh
/// install notifies without being configured first.
pub fn enabled(conn: &Connection, kind: ReminderKind) -> Result<bool> {
    Ok(settings::get(conn, &enabled_key(kind))?
        .map(|value| value != "false")
        .unwrap_or(true))
}

pub fn set_enabled(conn: &Connection, kind: ReminderKind, enabled: bool) -> Result<()> {
    settings::set(
        conn,
        &enabled_key(kind),
        if enabled { "true" } else { "false" },
    )
}

fn enabled_key(kind: ReminderKind) -> String {
    format!("notify.{}.enabled", kind.as_str())
}

const SELECT_REMINDER: &str = "SELECT id, kind, task_id, trigger_at, status, snoozed_until, system_notification_tag, session_id FROM reminders {filter}";

fn row_to_reminder(row: &Row) -> rusqlite::Result<Reminder> {
    let kind: String = row.get(1)?;
    let trigger_at: String = row.get(3)?;
    let status: String = row.get(4)?;
    let snoozed_until: Option<String> = row.get(5)?;

    Ok(Reminder {
        id: row.get(0)?,
        kind: ReminderKind::from_db_str(&kind).unwrap_or(ReminderKind::Custom),
        task_id: row.get(2)?,
        trigger_at: parse_rfc3339(&trigger_at),
        status: ReminderStatus::from_db_str(&status).unwrap_or(ReminderStatus::Pending),
        snoozed_until: snoozed_until.map(|value| parse_rfc3339(&value)),
        system_notification_tag: row.get(6)?,
        session_id: row.get(7)?,
    })
}

fn parse_rfc3339(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .expect("timestamps written by this crate are always valid RFC3339")
        .with_timezone(&Utc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db;
    use chrono::Duration;

    fn conn() -> Connection {
        db::open_in_memory().unwrap()
    }

    fn new_reminder(kind: ReminderKind, in_minutes: i64) -> NewReminder {
        NewReminder {
            kind,
            task_id: None,
            session_id: None,
            trigger_at: Utc::now() + Duration::minutes(in_minutes),
        }
    }

    #[test]
    fn create_starts_pending_and_roundtrips() {
        let conn = conn();
        let reminder = create(&conn, new_reminder(ReminderKind::Custom, 30)).unwrap();

        assert_eq!(reminder.kind, ReminderKind::Custom);
        assert_eq!(reminder.status, ReminderStatus::Pending);
        assert_eq!(reminder.task_id, None);
        assert!(reminder.snoozed_until.is_none());
        assert!(reminder.system_notification_tag.is_none());
    }

    #[test]
    fn outstanding_reminders_sort_before_settled_ones() {
        let conn = conn();
        let later = create(&conn, new_reminder(ReminderKind::Custom, 60)).unwrap();
        let sooner = create(&conn, new_reminder(ReminderKind::Custom, 10)).unwrap();
        let done = create(&conn, new_reminder(ReminderKind::Custom, 5)).unwrap();
        mark(&conn, done.id, ReminderStatus::Dismissed).unwrap();

        let listed = list(&conn).unwrap();

        assert_eq!(
            listed.iter().map(|r| r.id).collect::<Vec<_>>(),
            vec![sooner.id, later.id, done.id]
        );
    }

    #[test]
    fn mark_records_an_outcome() {
        let conn = conn();
        let reminder = create(&conn, new_reminder(ReminderKind::TaskDue, 5)).unwrap();

        let fired = mark(&conn, reminder.id, ReminderStatus::Fired).unwrap();

        assert_eq!(fired.status, ReminderStatus::Fired);
    }

    #[test]
    fn snoozing_rearms_the_reminder() {
        let conn = conn();
        let reminder = create(&conn, new_reminder(ReminderKind::Custom, 5)).unwrap();
        let until = Utc::now() + Duration::minutes(10);

        let snoozed = snooze(&conn, reminder.id, until).unwrap();

        assert_eq!(snoozed.status, ReminderStatus::Snoozed);
        assert_eq!(
            snoozed.snoozed_until.map(|t| t.timestamp()),
            Some(until.timestamp())
        );
        assert_eq!(snoozed.trigger_at.timestamp(), until.timestamp());
        // Still outstanding, so it is registered with the system again.
        assert_eq!(pending(&conn).unwrap().len(), 1);
    }

    #[test]
    fn rescheduling_returns_it_to_pending() {
        let conn = conn();
        let reminder = create(&conn, new_reminder(ReminderKind::Custom, 5)).unwrap();
        snooze(&conn, reminder.id, Utc::now() + Duration::minutes(10)).unwrap();

        let later = Utc::now() + Duration::hours(2);
        let moved = reschedule(&conn, reminder.id, later).unwrap();

        assert_eq!(moved.status, ReminderStatus::Pending);
        assert!(moved.snoozed_until.is_none());
        assert_eq!(moved.trigger_at.timestamp(), later.timestamp());
    }

    #[test]
    fn pending_excludes_settled_reminders() {
        let conn = conn();
        let keep = create(&conn, new_reminder(ReminderKind::Custom, 10)).unwrap();
        let drop = create(&conn, new_reminder(ReminderKind::Custom, 20)).unwrap();
        mark(&conn, drop.id, ReminderStatus::Dismissed).unwrap();

        let outstanding = pending(&conn).unwrap();

        assert_eq!(
            outstanding.iter().map(|r| r.id).collect::<Vec<_>>(),
            vec![keep.id]
        );
    }

    #[test]
    fn due_selects_only_reminders_whose_time_has_come() {
        let conn = conn();
        let past = create(&conn, new_reminder(ReminderKind::Custom, -10)).unwrap();
        create(&conn, new_reminder(ReminderKind::Custom, 30)).unwrap();

        let due_now = due(&conn, Utc::now()).unwrap();

        assert_eq!(
            due_now.iter().map(|r| r.id).collect::<Vec<_>>(),
            vec![past.id]
        );
    }

    #[test]
    fn the_system_tag_can_be_stored_and_cleared() {
        let conn = conn();
        let reminder = create(&conn, new_reminder(ReminderKind::Custom, 10)).unwrap();

        set_system_tag(&conn, reminder.id, Some("reminder-7")).unwrap();
        assert_eq!(
            get(&conn, reminder.id)
                .unwrap()
                .unwrap()
                .system_notification_tag,
            Some("reminder-7".to_string())
        );

        set_system_tag(&conn, reminder.id, None).unwrap();
        assert!(
            get(&conn, reminder.id)
                .unwrap()
                .unwrap()
                .system_notification_tag
                .is_none()
        );
    }

    #[test]
    fn a_reminder_can_be_found_by_its_system_tag() {
        let conn = conn();
        let reminder = create(&conn, new_reminder(ReminderKind::Custom, 10)).unwrap();
        set_system_tag(&conn, reminder.id, Some("reminder-42")).unwrap();

        let found = find_by_tag(&conn, "reminder-42").unwrap();

        assert_eq!(found.map(|r| r.id), Some(reminder.id));
        assert!(find_by_tag(&conn, "reminder-99").unwrap().is_none());
    }

    #[test]
    fn deleting_removes_the_row() {
        let conn = conn();
        let reminder = create(&conn, new_reminder(ReminderKind::Custom, 10)).unwrap();

        delete(&conn, reminder.id).unwrap();

        assert!(get(&conn, reminder.id).unwrap().is_none());
    }

    #[test]
    fn a_session_reminder_is_linked_to_its_session() {
        let conn = conn();
        let session =
            crate::storage::pomodoro::start(&conn, crate::domain::SessionKind::Work, None, 1500)
                .unwrap();

        let reminder = create_for_session(
            &conn,
            session.id,
            ReminderKind::PomodoroBreak,
            Utc::now() + Duration::minutes(25),
        )
        .unwrap();

        assert_eq!(reminder.session_id, Some(session.id));
        assert_eq!(reminder.kind, ReminderKind::PomodoroBreak);
    }

    #[test]
    fn withdrawing_a_session_settles_its_reminder() {
        let conn = conn();
        let session =
            crate::storage::pomodoro::start(&conn, crate::domain::SessionKind::Work, None, 1500)
                .unwrap();
        let reminder = create_for_session(
            &conn,
            session.id,
            ReminderKind::PomodoroBreak,
            Utc::now() + Duration::minutes(25),
        )
        .unwrap();

        withdraw_for_session(&conn, session.id).unwrap();

        // Still present, so the next sync can cancel it with the system.
        assert_eq!(
            get(&conn, reminder.id).unwrap().unwrap().status,
            ReminderStatus::Dismissed
        );
        assert!(pending(&conn).unwrap().is_empty());
    }

    #[test]
    fn every_kind_is_enabled_until_turned_off() {
        let conn = conn();
        for kind in [
            ReminderKind::TaskDue,
            ReminderKind::PriorityAlert,
            ReminderKind::PomodoroBreak,
            ReminderKind::Custom,
        ] {
            assert!(
                enabled(&conn, kind).unwrap(),
                "{kind:?} should default to on"
            );
        }

        set_enabled(&conn, ReminderKind::PomodoroBreak, false).unwrap();

        assert!(!enabled(&conn, ReminderKind::PomodoroBreak).unwrap());
        assert!(
            enabled(&conn, ReminderKind::TaskDue).unwrap(),
            "kinds are independent"
        );
    }
}
