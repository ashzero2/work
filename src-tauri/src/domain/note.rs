use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Queryable note metadata, kept in SQLite. The note's actual text lives in
/// its `.md` file (see `storage::notes_fs`) — this is the fast-access index.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteMeta {
    pub id: i64,
    pub title: String,
    pub file_path: String,
    pub linked_task_id: Option<i64>,
    pub color: String,
    pub rotation_deg: f64,
    pub pos_x: Option<f64>,
    pub pos_y: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// The YAML frontmatter written at the top of a note's `.md` file — the
/// source of truth, mirroring the queryable fields also kept in SQLite so
/// the index can be rebuilt from disk if it's ever lost.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteFrontMatter {
    pub id: i64,
    pub title: String,
    pub tags: Vec<String>,
    pub linked_task_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
