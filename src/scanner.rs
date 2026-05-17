use std::fs;
use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

use crate::action::Action;

pub fn scan_actions(path: &Path) -> Result<Vec<Action>> {
    let mut actions = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let Some(ext) = entry.path().extension() else {
            continue;
        };

        if ext != "toml" {
            continue;
        }

        let content = fs::read_to_string(entry.path())?;

        let action: Action = toml::from_str(&content)?;

        actions.push(action);
    }

    Ok(actions)
}
