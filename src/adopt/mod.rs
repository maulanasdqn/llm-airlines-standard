use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::check::files::{self, FileQuery};
use crate::disk::{enable_execution, write_bytes};
use crate::embedded::{find_file, get_embedded_text, list_files as list_embedded};
use crate::error::{Error, Result};
use crate::spec::load_embedded_dictionary;

pub const PROFILES: [&str; 6] = ["base", "docs", "typescript", "python", "go", "rust"];
const SHIM: &[u8] = b"@AGENTS.md\n";
const HOOK_SOURCE: &str = "linters/git/commit-msg";
const HOOK_TARGET: &str = ".git/hooks/commit-msg";

pub struct Request {
    pub profile: Option<String>,
    pub folder: PathBuf,
    pub has_linters: bool,
    pub has_hook: bool,
    pub is_dry_run: bool,
}

fn find_main_language(folder: &Path) -> Result<String> {
    let dictionary = load_embedded_dictionary()?;
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for (language, profile) in &dictionary.profiles {
        let query = FileQuery { extensions: &profile.extensions, root: folder, excluded: &[] };
        counts.insert(language.as_str(), files::list(&[folder.to_path_buf()], &query).len());
    }
    let top = counts.into_iter().filter(|(_, count)| *count > 0).max_by_key(|(_, count)| *count);
    Ok(top.map_or_else(|| String::from("base"), |(language, _)| language.to_string()))
}

fn list_linter_files(profile: &str) -> Vec<(String, &'static [u8])> {
    let prefix = match profile {
        "typescript" | "python" | "go" => format!("linters/{profile}/"),
        "rust" => String::from("linters/rust/clippy.toml"),
        _ => String::from("linters/vale/"),
    };
    let folder = prefix.rsplit_once('/').map_or("", |(head, _)| head);
    list_embedded(&prefix).into_iter().map(|(path, bytes)| (path[folder.len() + 1..].to_string(), bytes)).collect()
}

fn list_adopt_files(request: &Request, profile: &str) -> Result<Vec<(String, &'static [u8])>> {
    let agents = find_file(&format!("templates/agents/{profile}/AGENTS.md"))
        .ok_or_else(|| Error::new(format!("Unknown profile \"{profile}\". Use one of: {}.", PROFILES.join(", "))))?;
    let mut files = vec![
        (String::from("AGENTS.md"), agents),
        (String::from("CLAUDE.md"), SHIM),
        (String::from("LEXICON.md"), get_embedded_text("templates/LEXICON.md").as_bytes()),
    ];
    if request.has_linters {
        files.extend(list_linter_files(profile));
    }
    if request.has_hook {
        files.push((HOOK_TARGET.to_string(), get_embedded_text(HOOK_SOURCE).as_bytes()));
    }
    Ok(files)
}

fn write_file(request: &Request, relative: &str, bytes: &[u8]) -> Result<String> {
    let folder = &request.folder;
    let path = folder.join(relative);
    if path.exists() {
        return Ok(format!("Kept {relative}: the file exists. Merge the STE rules into it by hand."));
    }
    if relative == HOOK_TARGET && !folder.join(".git").is_dir() {
        return Ok(format!("Skipped {relative}: the folder is not a git repository."));
    }
    if request.is_dry_run {
        return Ok(format!("Dry run: write {relative}."));
    }
    write_bytes(&path, bytes)?;
    if relative == HOOK_TARGET {
        enable_execution(&path)?;
    }
    Ok(format!("Wrote {relative}."))
}

pub fn run(request: &Request) -> Result<u8> {
    let profile = request.profile.clone().map_or_else(|| find_main_language(&request.folder), Ok)?;
    println!("Profile: {profile}");
    for (relative, bytes) in list_adopt_files(request, &profile)? {
        println!("{}", write_file(request, &relative, bytes)?);
    }
    if request.has_linters && profile == "rust" {
        println!("Add this table to Cargo.toml:\n\n{}", get_embedded_text("linters/rust/lints.toml"));
    }
    println!("Next: run ste check lexicon . and move the real domain words into LEXICON.md.");
    Ok(0)
}
