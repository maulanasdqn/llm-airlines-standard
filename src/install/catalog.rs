use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::embedded::list_files;
use crate::spec::frontmatter;

static FIRST_SENTENCE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(.+?[.!?])(?:\s|$)").expect("the pattern is a valid regex"));

pub struct Skill {
    pub name: String,
    pub summary: String,
}

pub struct Target {
    pub key: &'static str,
    pub label: &'static str,
    pub folder: &'static str,
}

pub const TARGETS: [Target; 4] = [
    Target { key: "claude-user", label: "Claude Code, for your user account", folder: "~/.claude/skills" },
    Target { key: "claude-project", label: "Claude Code, for the current project", folder: ".claude/skills" },
    Target {
        key: "agents-user",
        label: "Codex, Cursor, and other agents, for your user account",
        folder: "~/.agents/skills",
    },
    Target {
        key: "agents-project",
        label: "Codex, Cursor, and other agents, for the current project",
        folder: ".agents/skills",
    },
];

fn find_summary(description: &str) -> String {
    FIRST_SENTENCE.captures(description).map_or_else(|| description.to_string(), |found| found[1].to_string())
}

pub fn list_skills() -> Vec<Skill> {
    list_files("skills/")
        .into_iter()
        .filter(|(path, _)| path.matches('/').count() == 2 && path.ends_with("/SKILL.md"))
        .map(|(path, bytes)| {
            let fields = frontmatter::parse(&String::from_utf8_lossy(bytes));
            let folder = path.split('/').nth(1).unwrap_or_default().to_string();
            Skill {
                name: fields.get("name").cloned().unwrap_or(folder),
                summary: find_summary(fields.get("description").map_or("", String::as_str)),
            }
        })
        .collect()
}

pub fn list_skill_files(name: &str) -> Vec<(String, &'static [u8])> {
    let prefix = format!("skills/{name}/");
    list_files(&prefix).into_iter().map(|(path, bytes)| (path[prefix.len()..].to_string(), bytes)).collect()
}

pub fn resolve_folder(folder: &str, project: &Path) -> PathBuf {
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_default();
    let expanded = folder.strip_prefix("~/").map_or_else(|| PathBuf::from(folder), |rest| home.join(rest));
    if expanded.is_absolute() {
        expanded
    } else {
        project.join(expanded)
    }
}
