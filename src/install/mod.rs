pub mod catalog;
pub mod copying;
pub mod selection;

use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use catalog::{list_skill_files, list_skills, resolve_folder, Skill, TARGETS};
use copying::{find_status, replace_skill, write_skill, Status};
use selection::{ask_choice, ask_confirmation, read_answer, Console, Menu};

const OTHER_FOLDER: &str = "Another folder (you type the path)";

pub struct Request {
    pub skills: Option<String>,
    pub targets: Vec<String>,
    pub folders: Vec<PathBuf>,
    pub project: PathBuf,
    pub is_forced: bool,
    pub is_dry_run: bool,
}

struct Session {
    console: Console,
    is_forced: bool,
    is_dry_run: bool,
    is_interactive: bool,
}

pub fn format_listing() -> String {
    let mut lines = vec![String::from("Skills:")];
    lines.extend(list_skills().iter().map(|skill| format!("  {}: {}", skill.name, skill.summary)));
    lines.extend([String::new(), String::from("Targets:")]);
    lines.extend(TARGETS.iter().map(|target| format!("  {}: {} ({})", target.key, target.label, target.folder)));
    lines.join("\n")
}

fn find_named_skills(text: &str, skills: Vec<Skill>) -> Result<Vec<Skill>> {
    let names: Vec<&str> = text.split(',').map(str::trim).filter(|name| !name.is_empty()).collect();
    if names == ["all"] {
        return Ok(skills);
    }
    let unknown: Vec<&str> =
        names.iter().copied().filter(|name| !skills.iter().any(|skill| skill.name == *name)).collect();
    if !unknown.is_empty() || names.is_empty() {
        let listed = if unknown.is_empty() { String::from("no name") } else { unknown.join(", ") };
        return Err(Error::new(format!("Unknown skill: {listed}. Run ste install --list to see the skill names.")));
    }
    Ok(skills.into_iter().filter(|skill| names.contains(&skill.name.as_str())).collect())
}

fn select_skills(request: &Request, console: &mut Console) -> Result<Vec<Skill>> {
    let skills = list_skills();
    if let Some(text) = &request.skills {
        return find_named_skills(text, skills);
    }
    let options: Vec<String> = skills.iter().map(|skill| format!("{}: {}", skill.name, skill.summary)).collect();
    let default: Vec<usize> = (0..skills.len()).collect();
    let indexes = ask_choice(
        console,
        &Menu { question: "Which skills do you want to install?", options: &options, default: &default },
    )?;
    Ok(skills.into_iter().enumerate().filter(|(index, _)| indexes.contains(index)).map(|(_, skill)| skill).collect())
}

fn select_folders(request: &Request, console: &mut Console) -> Result<Vec<PathBuf>> {
    let project = std::fs::canonicalize(&request.project).unwrap_or_else(|_| request.project.clone());
    let mut folders: Vec<PathBuf> = TARGETS
        .iter()
        .filter(|target| request.targets.iter().any(|key| key == target.key))
        .map(|target| resolve_folder(target.folder, &project))
        .collect();
    folders.extend(request.folders.iter().map(|folder| resolve_folder(&folder.to_string_lossy(), &project)));
    if !folders.is_empty() {
        return Ok(folders);
    }
    let mut options: Vec<String> =
        TARGETS.iter().map(|target| format!("{} ({})", target.label, target.folder)).collect();
    options.push(OTHER_FOLDER.to_string());
    let indexes = ask_choice(
        console,
        &Menu { question: "Where do you want to install them?", options: &options, default: &[0] },
    )?;
    folders = indexes
        .iter()
        .filter_map(|index| TARGETS.get(*index))
        .map(|target| resolve_folder(target.folder, &project))
        .collect();
    if indexes.contains(&TARGETS.len()) {
        folders.push(read_other_folder(console, &project)?);
    }
    Ok(folders)
}

fn read_other_folder(console: &mut Console, project: &Path) -> Result<PathBuf> {
    let answer = read_answer(console, "Type the path of the skills folder: ")?;
    if answer.trim().is_empty() {
        return Err(Error::new("The folder path is empty. Run the installer again and type a path."));
    }
    Ok(resolve_folder(answer.trim(), project))
}

fn can_replace(destination: &Path, session: &mut Session) -> Result<bool> {
    if session.is_forced || !session.is_interactive {
        return Ok(session.is_forced);
    }
    ask_confirmation(&mut session.console, &format!("{} has local changes. Replace it?", destination.display()))
}

fn install_skill(skill: &Skill, folder: &Path, session: &mut Session) -> Result<(bool, String)> {
    let destination = folder.join(&skill.name);
    let shown = destination.display();
    let files = list_skill_files(&skill.name);
    let status = find_status(&files, &destination);
    let skipped = |reason: String| Ok((false, format!("Skipped {}: {reason}", skill.name)));
    match status {
        Status::Unchanged => return skipped(format!("{shown} is up to date.")),
        Status::Blocked => {
            return skipped(format!(
                "{shown} has no SKILL.md file. Move or delete that folder, then run the installer again."
            ))
        }
        Status::Changed if !can_replace(&destination, session)? => {
            return skipped(format!("{shown} has local changes. Run the installer again with --force to replace it."));
        }
        _ => {}
    }
    let (action, done) = if status == Status::New { ("install", "Installed") } else { ("replace", "Replaced") };
    if session.is_dry_run {
        return Ok((false, format!("Dry run: {action} {} in {shown}.", skill.name)));
    }
    if status == Status::New {
        write_skill(&files, &destination)?
    } else {
        replace_skill(&files, &destination)?
    }
    Ok((true, format!("{done} {} in {shown}.", skill.name)))
}

pub fn run(request: &Request, console: Console) -> Result<u8> {
    let is_interactive = request.skills.is_none() || (request.targets.is_empty() && request.folders.is_empty());
    let mut session = Session { console, is_forced: request.is_forced, is_dry_run: request.is_dry_run, is_interactive };
    let skills = select_skills(request, &mut session.console)?;
    let folders = select_folders(request, &mut session.console)?;
    let mut has_changes = false;
    for folder in &folders {
        for skill in &skills {
            let (is_changed, message) = install_skill(skill, folder, &mut session)?;
            has_changes |= is_changed;
            println!("{message}");
        }
    }
    if has_changes {
        println!("Start a new agent session to load the skills.");
    }
    Ok(0)
}
