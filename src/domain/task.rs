use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    None,
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn as_i64(self) -> i64 {
        match self {
            Priority::None => 0,
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
        }
    }

    pub fn from_i64(value: i64) -> Self {
        match value {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            _ => Priority::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatRule {
    Daily,
    Weekly,
    Monthly,
}

impl RepeatRule {
    pub fn as_str(self) -> &'static str {
        match self {
            RepeatRule::Daily => "daily",
            RepeatRule::Weekly => "weekly",
            RepeatRule::Monthly => "monthly",
        }
    }

    pub fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "daily" => Some(RepeatRule::Daily),
            "weekly" => Some(RepeatRule::Weekly),
            "monthly" => Some(RepeatRule::Monthly),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub column_id: i64,
    pub position: f64,
    pub priority: Priority,
    pub due_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub repeat_rule: Option<RepeatRule>,
    pub parent_task_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Fields needed to create a task — separate from `Task` since callers
/// shouldn't supply an id/position/timestamps themselves.
#[derive(Debug, Clone)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub column_id: i64,
    pub priority: Priority,
    pub due_at: Option<DateTime<Utc>>,
    pub repeat_rule: Option<RepeatRule>,
    pub parent_task_id: Option<i64>,
}
