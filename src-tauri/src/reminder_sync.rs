use chrono::Utc;
use rusqlite::Connection;

use crate::domain::{Reminder, ReminderKind, ReminderStatus};
use crate::error::Result;
use crate::storage::reminders;
use crate::storage::tasks;

/// Run at launch. Anything whose time passed while the app was closed has
/// already been announced by the OS — its notification was registered before
/// delivery, so it needs no re-posting, and re-posting a past-due reminder would
/// re-notify on every launch. Settle those, then sync the rest.
pub fn reconcile(conn: &Connection) -> Result<()> {
    for reminder in reminders::due(conn, Utc::now())? {
        reminders::mark(conn, reminder.id, ReminderStatus::Fired)?;
    }
    sync(conn)
}

/// Re-registers the outstanding reminders with the OS and cancels anything no
/// longer wanted, so the system's pending set always matches the database.
/// Re-posting an identifier replaces that request, which keeps this idempotent.
///
/// Run after every edit: the OS owns delivery, so it has to be told about
/// changes rather than discovering them.
pub fn sync(conn: &Connection) -> Result<()> {
    let all = reminders::list(conn)?;

    if !crate::platform::notifications_available() {
        return Ok(());
    }

    let now = Utc::now();
    for reminder in &all {
        let wanted = matches!(
            reminder.status,
            ReminderStatus::Pending | ReminderStatus::Snoozed
        ) && reminders::enabled(conn, reminder.kind)?;
        let tag = tag_for(reminder.id);

        if !wanted {
            if reminder.system_notification_tag.is_some() {
                crate::platform::cancel_notifications(std::slice::from_ref(&tag));
                reminders::set_system_tag(conn, reminder.id, None)?;
            }
            continue;
        }

        let delay = (reminder.trigger_at - now).num_seconds().max(0) as f64;
        crate::platform::post_notification(
            &tag,
            &title_for(reminder.kind),
            &body_for(conn, reminder)?,
            delay,
        );
        reminders::set_system_tag(conn, reminder.id, Some(&tag))?;
    }

    Ok(())
}

/// The identifier a reminder is registered under. Stable across syncs so
/// re-posting replaces rather than duplicates.
fn tag_for(id: i64) -> String {
    format!("reminder-{id}")
}

fn title_for(kind: ReminderKind) -> String {
    match kind {
        ReminderKind::TaskDue => "Task due",
        ReminderKind::PriorityAlert => "Priority task",
        ReminderKind::PomodoroBreak => "Break time",
        ReminderKind::Custom => "Reminder",
    }
    .to_string()
}

fn body_for(conn: &Connection, reminder: &Reminder) -> Result<String> {
    let Some(task_id) = reminder.task_id else {
        return Ok("Open Work Dashboard".to_string());
    };
    Ok(tasks::get(conn, task_id)?
        .map_or_else(|| "Open Work Dashboard".to_string(), |task| task.title))
}
