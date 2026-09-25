use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionKind {
    Work,
    Break,
    LongBreak,
}

impl SessionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            SessionKind::Work => "work",
            SessionKind::Break => "break",
            SessionKind::LongBreak => "long_break",
        }
    }

    pub fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "work" => Some(SessionKind::Work),
            "break" => Some(SessionKind::Break),
            "long_break" => Some(SessionKind::LongBreak),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PomodoroSession {
    pub id: i64,
    pub task_id: Option<i64>,
    pub kind: SessionKind,
    pub planned_seconds: i64,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub completed: bool,
}
