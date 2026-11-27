use std::path::Path;

use super::plan::{Outputs, EXECUTABLE_NAMES, INSTRUCTION_NAMES, OWNED_FOLDERS};
use crate::disk::{enable_execution, list_tree, write_bytes};
use crate::error::Result;

const TOP_LEVEL_PARTS: usize = 2;

fn is_owned(relative: &str) -> bool {
    let parts: Vec<&str> = relative.split('/').collect();
    let is_source = relative.starts_with("linters/src/");
    let is_instruction = parts.len() == TOP_LEVEL_PARTS && INSTRUCTION_NAMES.contains(&parts[1]);
    let is_cache = parts.iter().any(|part| *part == "__pycache__" || *part == ".DS_Store");
    OWNED_FOLDERS.iter().any(|folder| relative.starts_with(&format!("{folder}/")))
        && !is_source
        && !is_instruction
        && !is_cache
}

pub fn list_stale(root: &Path, outputs: &Outputs) -> Vec<String> {
    OWNED_FOLDERS
        .iter()
        .flat_map(|folder| {
            list_tree(&root.join(folder)).into_iter().map(move |relative| format!("{folder}/{relative}"))
        })
        .filter(|relative| is_owned(relative) && !outputs.contains_key(relative))
        .collect()
}

pub fn find_drift(root: &Path, outputs: &Outputs) -> Vec<String> {
    let mut drift: Vec<String> = outputs
        .iter()
        .filter_map(|(relative, content)| match std::fs::read(root.join(relative)) {
            Err(_) => Some(format!("missing: {relative}")),
            Ok(existing) if existing != *content => Some(format!("changed: {relative}")),
            Ok(_) => None,
        })
        .collect();
    drift.extend(list_stale(root, outputs).into_iter().map(|relative| format!("stale: {relative}")));
    drift
}

pub fn write(root: &Path, outputs: &Outputs) -> Result<Vec<String>> {
    let mut written = Vec::new();
    for (relative, content) in outputs {
        let path = root.join(relative);
        if std::fs::read(&path).is_ok_and(|existing| existing == *content) {
            continue;
        }
        write_bytes(&path, content)?;
        if EXECUTABLE_NAMES.iter().any(|name| relative.ends_with(name)) {
            enable_execution(&path)?;
        }
        written.push(relative.clone());
    }
    for relative in list_stale(root, outputs) {
        std::fs::remove_file(root.join(&relative))?;
        written.push(format!("deleted {relative}"));
    }
    Ok(written)
}
