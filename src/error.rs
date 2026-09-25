use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("migration error: {0}")]
    Migration(#[from] rusqlite_migration::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("frontmatter error: {0}")]
    FrontMatter(String),

    #[error("column still has tasks; move them out before deleting")]
    ColumnNotEmpty,

    #[error("not found")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, AppError>;
