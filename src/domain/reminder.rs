use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReminderKind {
    TaskDue,
    PriorityAlert,
    PomodoroBreak,
    Custom,
}

impl ReminderKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ReminderKind::TaskDue => "task_due",
            ReminderKind::PriorityAlert => "priority_alert",
            ReminderKind::PomodoroBreak => "pomodoro_break",
            ReminderKind::Custom => "custom",
        }
    }

    pub fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "task_due" => Some(ReminderKind::TaskDue),
            "priority_alert" => Some(ReminderKind::PriorityAlert),
            "pomodoro_break" => Some(ReminderKind::PomodoroBreak),
            "custom" => Some(ReminderKind::Custom),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReminderStatus {
    Pending,
    Fired,
    Snoozed,
    Dismissed,
}

impl ReminderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ReminderStatus::Pending => "pending",
            ReminderStatus::Fired => "fired",
            ReminderStatus::Snoozed => "snoozed",
            ReminderStatus::Dismissed => "dismissed",
        }
    }

    pub fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(ReminderStatus::Pending),
            "fired" => Some(ReminderStatus::Fired),
            "snoozed" => Some(ReminderStatus::Snoozed),
            "dismissed" => Some(ReminderStatus::Dismissed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: i64,
    pub kind: ReminderKind,
    pub task_id: Option<i64>,
    pub trigger_at: DateTime<Utc>,
    pub status: ReminderStatus,
    pub snoozed_until: Option<DateTime<Utc>>,
    pub system_notification_tag: Option<String>,
}
