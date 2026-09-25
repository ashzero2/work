use rusqlite_migration::{M, Migrations};

const INITIAL_SCHEMA: &str = r#"
CREATE TABLE columns (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    order_index REAL NOT NULL,
    color TEXT,
    wip_limit INTEGER
);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    column_id INTEGER NOT NULL REFERENCES columns(id),
    position REAL NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    due_at TEXT,
    completed_at TEXT,
    repeat_rule TEXT,
    parent_task_id INTEGER REFERENCES tasks(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE notes (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    file_path TEXT NOT NULL,
    linked_task_id INTEGER REFERENCES tasks(id),
    color TEXT NOT NULL,
    rotation_deg REAL NOT NULL,
    pos_x REAL,
    pos_y REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Standalone FTS5 table (stores its own copy of title/body, not external-content)
-- so it can support snippet()/highlight() directly. Kept in sync explicitly by
-- storage::notes_index rather than via triggers, since a note's body lives in a
-- markdown file, not a column a SQLite trigger could see.
CREATE VIRTUAL TABLE notes_fts USING fts5(title, body);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    color TEXT
);

CREATE TABLE taggables (
    tag_id INTEGER NOT NULL REFERENCES tags(id),
    entity_type TEXT NOT NULL,
    entity_id INTEGER NOT NULL,
    PRIMARY KEY (tag_id, entity_type, entity_id)
);

CREATE TABLE pomodoro_sessions (
    id INTEGER PRIMARY KEY,
    task_id INTEGER REFERENCES tasks(id),
    kind TEXT NOT NULL,
    planned_seconds INTEGER NOT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    completed INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE reminders (
    id INTEGER PRIMARY KEY,
    kind TEXT NOT NULL,
    task_id INTEGER REFERENCES tasks(id),
    trigger_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    snoozed_until TEXT,
    system_notification_tag TEXT
);

CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
"#;

const MIGRATIONS_SLICE: &[M<'_>] = &[M::up(INITIAL_SCHEMA)];

pub const MIGRATIONS: Migrations<'_> = Migrations::from_slice(MIGRATIONS_SLICE);
