use std::path::Path;

use crate::disk::{list_tree, write_bytes};
use crate::error::{Error, Result};

const SKILL_FILE: &str = "SKILL.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Unchanged,
    Changed,
    Blocked,
}

fn is_ignored(relative: &str) -> bool {
    relative.split('/').any(|part| part == "__pycache__" || part == ".DS_Store") || relative.ends_with(".pyc")
}

pub fn find_status(files: &[(String, &[u8])], destination: &Path) -> Status {
    if !destination.exists() {
        return Status::New;
    }
    if !destination.join(SKILL_FILE).is_file() {
        return Status::Blocked;
    }
    let existing: Vec<String> = list_tree(destination).into_iter().filter(|relative| !is_ignored(relative)).collect();
    let mut expected: Vec<&String> = files.iter().map(|(relative, _)| relative).collect();
    expected.sort();
    let is_same_set = existing.iter().collect::<Vec<&String>>() == expected;
    let is_same_content = files
        .iter()
        .all(|(relative, bytes)| std::fs::read(destination.join(relative)).is_ok_and(|content| content == *bytes));
    if is_same_set && is_same_content {
        Status::Unchanged
    } else {
        Status::Changed
    }
}

pub fn write_skill(files: &[(String, &[u8])], destination: &Path) -> Result<()> {
    for (relative, bytes) in files {
        write_bytes(&destination.join(relative), bytes)?;
    }
    Ok(())
}

pub fn replace_skill(files: &[(String, &[u8])], destination: &Path) -> Result<()> {
    if !destination.join(SKILL_FILE).is_file() {
        return Err(Error::new(format!(
            "Cannot replace {}: the folder has no {SKILL_FILE} file.",
            destination.display()
        )));
    }
    std::fs::remove_dir_all(destination)
        .map_err(|error| Error::new(format!("Cannot remove {}: {error}.", destination.display())))?;
    write_skill(files, destination)
}
