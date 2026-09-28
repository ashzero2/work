use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rusqlite::Connection;
use serde::Serialize;

use crate::error::Result;
use crate::storage::{columns, tasks};

/// A task flattened for export. The column is resolved to its name so the file
/// reads on its own, and absent values stay absent rather than becoming the
/// string "null".
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportTask {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub column: String,
    pub priority: String,
    pub due_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub repeat_rule: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn tasks(conn: &Connection) -> Result<Vec<ExportTask>> {
    let names: HashMap<i64, String> = columns::list(conn)?
        .into_iter()
        .map(|column| (column.id, column.name))
        .collect();

    let rows = tasks::list_all(conn)?
        .into_iter()
        .map(|task| ExportTask {
            column: names
                .get(&task.column_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string()),
            priority: task.priority.as_str().to_string(),
            repeat_rule: task.repeat_rule.map(|rule| rule.as_str().to_string()),
            id: task.id,
            title: task.title,
            description: task.description,
            due_at: task.due_at,
            completed_at: task.completed_at,
            created_at: task.created_at,
            updated_at: task.updated_at,
        })
        .collect();

    Ok(rows)
}

pub fn to_json(rows: &[ExportTask]) -> Result<String> {
    Ok(serde_json::to_string_pretty(rows)?)
}

const HEADERS: [&str; 10] = [
    "id",
    "title",
    "description",
    "column",
    "priority",
    "dueAt",
    "completedAt",
    "repeatRule",
    "createdAt",
    "updatedAt",
];

pub fn to_csv(rows: &[ExportTask]) -> String {
    let mut out = HEADERS.join(",");
    out.push('\n');

    for row in rows {
        let fields = [
            row.id.to_string(),
            row.title.clone(),
            row.description.clone().unwrap_or_default(),
            row.column.clone(),
            row.priority.clone(),
            moment(row.due_at),
            moment(row.completed_at),
            row.repeat_rule.clone().unwrap_or_default(),
            row.created_at.to_rfc3339(),
            row.updated_at.to_rfc3339(),
        ];
        out.push_str(
            &fields
                .iter()
                .map(|field| csv_field(field))
                .collect::<Vec<_>>()
                .join(","),
        );
        out.push('\n');
    }

    out
}

/// Quotes a CSV field only when it needs it, doubling any inner quotes — the
/// minimum the format requires, rather than quoting everything defensively.
fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn moment(value: Option<DateTime<Utc>>) -> String {
    value.map(|at| at.to_rfc3339()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{NewColumn, NewTask, Priority, RepeatRule};
    use crate::storage::db;

    fn sample() -> ExportTask {
        ExportTask {
            id: 4,
            title: "Ship it".to_string(),
            description: None,
            column: "Doing".to_string(),
            priority: "high".to_string(),
            due_at: None,
            completed_at: None,
            repeat_rule: None,
            created_at: DateTime::parse_from_rfc3339("2026-09-28T09:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2026-09-28T09:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        }
    }

    #[test]
    fn a_task_is_exported_with_its_column_name_rather_than_its_id() {
        let conn = db::open_in_memory().unwrap();
        let column = columns::create(
            &conn,
            NewColumn {
                name: "Doing".into(),
                color: None,
                wip_limit: None,
            },
        )
        .unwrap();
        tasks::create(
            &conn,
            NewTask {
                title: "Ship it".into(),
                description: None,
                column_id: column.id,
                priority: Priority::High,
                due_at: None,
                repeat_rule: Some(RepeatRule::Weekly),
                parent_task_id: None,
            },
        )
        .unwrap();

        let rows = tasks(&conn).unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].column, "Doing");
        assert_eq!(rows[0].priority, "high");
        assert_eq!(rows[0].repeat_rule.as_deref(), Some("weekly"));
    }

    #[test]
    fn csv_quotes_only_what_needs_quoting() {
        assert_eq!(csv_field("plain"), "plain");
        assert_eq!(csv_field("a,b"), "\"a,b\"");
        assert_eq!(csv_field("say \"hi\""), "\"say \"\"hi\"\"\"");
        assert_eq!(csv_field("two\nlines"), "\"two\nlines\"");
    }

    #[test]
    fn csv_leaves_absent_values_empty_instead_of_writing_null() {
        let csv = to_csv(&[sample()]);

        assert!(!csv.contains("null"));
        // description, dueAt, completedAt and repeatRule are all absent, so they
        // land as empty fields rather than being filled in with anything.
        assert_eq!(
            csv.lines().nth(1).unwrap(),
            "4,Ship it,,Doing,high,,,,2026-09-28T09:00:00+00:00,2026-09-28T09:00:00+00:00"
        );
    }

    #[test]
    fn csv_carries_a_title_with_a_comma_and_a_quote_through_intact() {
        let row = ExportTask {
            title: "Buy milk, eggs and \"bread\"".to_string(),
            ..sample()
        };

        let csv = to_csv(&[row]);

        assert!(csv.contains("\"Buy milk, eggs and \"\"bread\"\"\""));
    }

    #[test]
    fn json_keeps_absent_values_as_null_and_round_trips() {
        let json = to_json(&[sample()]).unwrap();

        assert!(json.contains("\"dueAt\": null"));
        assert!(json.contains("\"column\": \"Doing\""));

        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed[0]["id"], 4);
        assert_eq!(parsed[0]["title"], "Ship it");
    }
}
