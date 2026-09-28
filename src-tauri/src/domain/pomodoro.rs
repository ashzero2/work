use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

/// The phase that follows `current`, given how many work sessions have run since
/// the last long break. Work earns a long break on the configured interval;
/// either kind of break hands back to work.
pub fn next_kind(
    current: SessionKind,
    completed_work_sessions: i64,
    sessions_per_long_break: i64,
) -> SessionKind {
    match current {
        SessionKind::Work => {
            let interval = sessions_per_long_break.max(1);
            if completed_work_sessions > 0 && completed_work_sessions % interval == 0 {
                SessionKind::LongBreak
            } else {
                SessionKind::Break
            }
        }
        SessionKind::Break | SessionKind::LongBreak => SessionKind::Work,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroSession {
    pub id: i64,
    pub task_id: Option<i64>,
    pub kind: SessionKind,
    pub planned_seconds: i64,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub completed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_hands_over_to_a_short_break_before_the_interval() {
        for completed in [1, 2, 3] {
            assert_eq!(
                next_kind(SessionKind::Work, completed, 4),
                SessionKind::Break,
                "after {completed} work sessions"
            );
        }
    }

    #[test]
    fn work_earns_a_long_break_on_the_interval() {
        for completed in [4, 8, 12] {
            assert_eq!(
                next_kind(SessionKind::Work, completed, 4),
                SessionKind::LongBreak,
                "after {completed} work sessions"
            );
        }
    }

    #[test]
    fn breaks_always_hand_back_to_work() {
        assert_eq!(next_kind(SessionKind::Break, 2, 4), SessionKind::Work);
        assert_eq!(next_kind(SessionKind::LongBreak, 4, 4), SessionKind::Work);
    }

    #[test]
    fn an_unset_interval_never_divides_by_zero() {
        assert_eq!(next_kind(SessionKind::Work, 1, 0), SessionKind::LongBreak);
        assert_eq!(next_kind(SessionKind::Work, 0, 0), SessionKind::Break);
    }

    #[test]
    fn kind_roundtrips_through_its_db_string() {
        for kind in [
            SessionKind::Work,
            SessionKind::Break,
            SessionKind::LongBreak,
        ] {
            assert_eq!(SessionKind::from_db_str(kind.as_str()), Some(kind));
        }
        assert_eq!(SessionKind::from_db_str("nonsense"), None);
    }
}
