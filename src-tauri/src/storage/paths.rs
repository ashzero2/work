use std::path::PathBuf;

/// The app's data directory (`~/tittle` by default).
pub fn data_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tittle")
}

pub fn db_path() -> PathBuf {
    data_dir().join("app.db")
}

pub fn notes_dir() -> PathBuf {
    data_dir().join("notes")
}
