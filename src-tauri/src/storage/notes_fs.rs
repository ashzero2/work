use std::fs;
use std::path::{Path, PathBuf};

use gray_matter::Matter;
use gray_matter::engine::YAML;

use crate::domain::NoteFrontMatter;
use crate::error::{AppError, Result};

/// Reads a note file, returning its parsed frontmatter and body separately.
pub fn read(path: &Path) -> Result<(NoteFrontMatter, String)> {
    let raw = fs::read_to_string(path)?;
    let matter = Matter::<YAML>::new();
    let parsed = matter
        .parse::<NoteFrontMatter>(&raw)
        .map_err(|e| AppError::FrontMatter(e.to_string()))?;
    let front_matter = parsed
        .data
        .ok_or_else(|| AppError::FrontMatter("note file is missing frontmatter".into()))?;
    Ok((front_matter, parsed.content))
}

/// Writes a note file as `---\n<yaml frontmatter>\n---\n<body>` — the
/// on-disk source of truth for the note.
pub fn write(path: &Path, front_matter: &NoteFrontMatter, body: &str) -> Result<()> {
    let yaml =
        serde_yaml_ng::to_string(front_matter).map_err(|e| AppError::FrontMatter(e.to_string()))?;
    let contents = format!("---\n{yaml}---\n{body}");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}

pub fn note_path(notes_dir: &Path, file_name: &str) -> PathBuf {
    notes_dir.join(file_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn write_then_read_roundtrips_frontmatter_and_body() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test-note.md");

        let front_matter = NoteFrontMatter {
            id: 1,
            title: "Test note".into(),
            tags: vec!["work".into(), "q3".into()],
            linked_task_id: Some(42),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let body = "Some **markdown** body.\n\n- item one\n- item two";

        write(&path, &front_matter, body).unwrap();
        let (read_front_matter, read_body) = read(&path).unwrap();

        assert_eq!(read_front_matter.title, "Test note");
        assert_eq!(read_front_matter.tags, vec!["work", "q3"]);
        assert_eq!(read_front_matter.linked_task_id, Some(42));
        assert_eq!(read_body.trim(), body.trim());
    }
}
