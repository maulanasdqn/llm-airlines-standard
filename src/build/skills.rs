use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::spec::frontmatter;

static NAME_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").expect("the pattern is a valid regex"));
const MAX_NAME_CHARS: usize = 64;
const MAX_DESCRIPTION_CHARS: usize = 1024;
const MAX_SKILL_LINES: usize = 500;

fn validate_skill(folder: &Path) -> Vec<String> {
    let path = folder.join("SKILL.md");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return vec![format!("{}: the folder has no SKILL.md file.", folder.display())];
    };
    let fields = frontmatter::parse(&text);
    let name = fields.get("name").cloned().unwrap_or_default();
    let description = fields.get("description").cloned().unwrap_or_default();
    let folder_name = folder.file_name().map(|value| value.to_string_lossy().to_string()).unwrap_or_default();
    let mut problems = Vec::new();
    if name != folder_name {
        problems.push(format!("{}: the name \"{name}\" must be the folder name \"{folder_name}\".", path.display()));
    }
    if !NAME_PATTERN.is_match(&name) || name.len() > MAX_NAME_CHARS {
        problems.push(format!(
            "{}: the name must use lowercase letters, digits, and hyphens, in {MAX_NAME_CHARS} characters or fewer.",
            path.display()
        ));
    }
    if description.is_empty() || description.chars().count() > MAX_DESCRIPTION_CHARS {
        problems
            .push(format!("{}: the description must have 1 to {MAX_DESCRIPTION_CHARS} characters.", path.display()));
    }
    if text.lines().count() >= MAX_SKILL_LINES {
        problems.push(format!(
            "{}: SKILL.md must have fewer than {MAX_SKILL_LINES} lines. Move detail to references.",
            path.display()
        ));
    }
    problems
}

pub fn validate_all(root: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(root.join("skills")) else {
        return vec![String::from("The repository has no skills folder.")];
    };
    let mut folders: Vec<_> = entries.flatten().map(|entry| entry.path()).filter(|path| path.is_dir()).collect();
    folders.sort();
    folders.iter().flat_map(|folder| validate_skill(folder)).collect()
}
