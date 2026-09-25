use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

    /// The next occurrence's due date after completing an instance due at `from`.
    pub fn next_occurrence(self, from: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            RepeatRule::Daily => from + chrono::Duration::days(1),
            RepeatRule::Weekly => from + chrono::Duration::weeks(1),
            RepeatRule::Monthly => from
                .checked_add_months(chrono::Months::new(1))
                .unwrap_or(from),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub column_id: i64,
    pub priority: Priority,
    pub due_at: Option<DateTime<Utc>>,
    pub repeat_rule: Option<RepeatRule>,
    pub parent_task_id: Option<i64>,
}
