use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const EMBEDDED_ROOTS: [&str; 7] = [
    "spec/writing/rules.md",
    "spec/code/rules.md",
    "spec/dictionary.json",
    "skills",
    "templates/agents",
    "templates/LEXICON.md",
    "linters",
];
const SKIPPED_PATHS: [&str; 4] = ["skills/AGENTS.md", "skills/CLAUDE.md", "linters/AGENTS.md", "linters/CLAUDE.md"];

fn is_skipped(relative: &str) -> bool {
    let is_cache = relative.split('/').any(|part| part == "__pycache__" || part == ".DS_Store");
    is_cache || relative.starts_with("linters/src/") || SKIPPED_PATHS.contains(&relative)
}

fn list_entry(path: &Path, files: &mut Vec<PathBuf>) {
    if path.is_file() {
        files.push(path.to_path_buf());
        return;
    }
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        list_entry(&entry.path(), files);
    }
}

fn to_relative(root: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    relative.to_string_lossy().replace('\\', "/")
}

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    for entry in EMBEDDED_ROOTS {
        println!("cargo:rerun-if-changed={entry}");
        list_entry(&root.join(entry), &mut files);
    }
    let mut rows: Vec<(String, PathBuf)> = files
        .into_iter()
        .map(|path| (to_relative(&root, &path), path))
        .filter(|(relative, _)| !is_skipped(relative))
        .collect();
    rows.sort();
    let lines: Vec<String> = rows
        .iter()
        .map(|(relative, path)| format!("    ({relative:?}, include_bytes!({:?})),", path.display().to_string()))
        .collect();
    let source = format!("pub static FILES: &[(&str, &[u8])] = &[\n{}\n];\n", lines.join("\n"));
    let target = PathBuf::from(env::var("OUT_DIR").expect("cargo sets OUT_DIR")).join("embedded.rs");
    fs::write(target, source).expect("the build script can write to OUT_DIR");
}
