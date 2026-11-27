use std::fs;
use std::path::Path;

use crate::error::{Error, Result};

pub fn read_text(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(|error| Error::new(format!("Cannot read {}: {error}.", path.display())))
}

pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).map_err(|error| Error::new(format!("Cannot read {}: {error}.", path.display())))
}

pub fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| Error::new(format!("Cannot create {}: {error}.", parent.display())))?;
    }
    fs::write(path, bytes).map_err(|error| Error::new(format!("Cannot write {}: {error}.", path.display())))
}

pub fn list_tree(folder: &Path) -> Vec<String> {
    let mut found = Vec::new();
    add_tree_files(folder, folder, &mut found);
    found.sort();
    found
}

fn add_tree_files(root: &Path, folder: &Path, found: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(folder) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            add_tree_files(root, &path, found);
        } else if let Ok(relative) = path.strip_prefix(root) {
            found.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

#[cfg(unix)]
pub fn enable_execution(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(permissions.mode() | 0o111);
    fs::set_permissions(path, permissions)
        .map_err(|error| Error::new(format!("Cannot change {}: {error}.", path.display())))
}

#[cfg(not(unix))]
pub fn enable_execution(_path: &Path) -> Result<()> {
    Ok(())
}
