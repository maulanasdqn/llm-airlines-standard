use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

const SKIPPED_FOLDERS: [&str; 13] = [
    ".git",
    ".hg",
    ".venv",
    "venv",
    "node_modules",
    "vendor",
    "target",
    "dist",
    "build",
    "__pycache__",
    ".mypy_cache",
    ".ruff_cache",
    ".pytest_cache",
];

pub struct FileQuery<'a> {
    pub extensions: &'a [String],
    pub root: &'a Path,
    pub excluded: &'a [String],
}

pub fn to_display_path(path: &Path) -> String {
    let current = std::env::current_dir().ok().and_then(|folder| fs::canonicalize(folder).ok());
    let absolute = fs::canonicalize(path).ok();
    let relative =
        current.zip(absolute).and_then(|(folder, file)| file.strip_prefix(folder).ok().map(Path::to_path_buf));
    relative.unwrap_or_else(|| path.to_path_buf()).to_string_lossy().replace('\\', "/")
}

fn build_glob(pattern: &str) -> Option<Regex> {
    let escaped = regex::escape(pattern).replace(r"\*", ".*").replace(r"\?", ".");
    Regex::new(&format!("^{escaped}$")).ok()
}

fn is_excluded(path: &Path, query: &FileQuery) -> bool {
    let absolute = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let relative = absolute.strip_prefix(query.root).map_or_else(|_| path.to_path_buf(), Path::to_path_buf);
    let text = relative.to_string_lossy().replace('\\', "/");
    query.excluded.iter().filter_map(|pattern| build_glob(pattern)).any(|glob| glob.is_match(&text))
}

fn list_folder(folder: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(folder) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths {
        let name = path.file_name().map(|name| name.to_string_lossy().to_string()).unwrap_or_default();
        if path.is_dir() && !SKIPPED_FOLDERS.contains(&name.as_str()) {
            list_folder(&path, found);
        } else if path.is_file() {
            found.push(path);
        }
    }
}

fn has_extension(path: &Path, extensions: &[String]) -> bool {
    let extension = path.extension().map(|value| format!(".{}", value.to_string_lossy().to_lowercase()));
    extension.is_some_and(|value| extensions.contains(&value))
}

pub fn list(paths: &[PathBuf], query: &FileQuery) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for path in paths {
        let mut candidates = Vec::new();
        if path.is_dir() {
            list_folder(path, &mut candidates);
        } else {
            candidates.push(path.clone());
        }
        found.extend(
            candidates
                .into_iter()
                .filter(|candidate| has_extension(candidate, query.extensions) && !is_excluded(candidate, query)),
        );
    }
    found
}
