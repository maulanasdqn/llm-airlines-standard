mod common;

use std::path::Path;
use std::process::{Command, Output};

use common::{get_root, get_stdout, run_ste, write_files, BINARY};
use ste::install::selection::parse;

const ALL_SKILLS: [&str; 4] = ["ste-adopt", "ste-code", "ste-review", "ste-writing"];

fn list_installed(folder: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(folder)
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| entry.path().join("SKILL.md").is_file())
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

fn run_install(arguments: &[&str], stdin: &str) -> Output {
    run_ste(&[&["install"], arguments].concat(), stdin)
}

fn run_script(arguments: &[&str], environment: &[(&str, &str)]) -> Output {
    let mut command = Command::new("bash");
    command.arg(get_root().join("install.sh")).args(arguments);
    for (key, value) in environment {
        command.env(key, value);
    }
    command.output().expect("bash runs install.sh")
}

#[test]
fn test_parse_numbers_and_defaults() {
    assert_eq!(parse("1 3", 4, &[0]).expect("the selection is valid"), [0, 2]);
    assert_eq!(parse("3,1,3", 4, &[0]).expect("the selection is valid"), [2, 0]);
    assert_eq!(parse("", 4, &[1]).expect("the selection is valid"), [1]);
    assert_eq!(parse("all", 3, &[0]).expect("the selection is valid"), [0, 1, 2]);
    for answer in ["0", "5", "two"] {
        assert!(parse(answer, 4, &[0]).is_err(), "{answer}");
    }
}

#[test]
fn test_list_names_every_skill() {
    let text = get_stdout(&run_install(&["--list"], ""));
    assert!(ALL_SKILLS.iter().all(|name| text.contains(name)));
}

#[test]
fn test_install_only_the_selected_skills() {
    let folder = tempfile::tempdir().expect("the test creates a folder");
    let target = folder.path().to_string_lossy().to_string();
    assert!(run_install(&["--skills", "ste-writing,ste-review", "--dir", &target], "").status.success());
    assert_eq!(list_installed(folder.path()), ["ste-review", "ste-writing"]);
}

#[test]
fn test_interactive_answers() {
    let folder = tempfile::tempdir().expect("the test creates a folder");
    let answers = format!("2 4\n5\n{}\n", folder.path().display());
    assert!(run_install(&[], &answers).status.success());
    assert_eq!(list_installed(folder.path()), ["ste-code", "ste-writing"]);
}

#[test]
fn test_second_install_changes_nothing() {
    let folder = tempfile::tempdir().expect("the test creates a folder");
    let target = folder.path().to_string_lossy().to_string();
    run_install(&["--skills", "all", "--dir", &target], "");
    let text = get_stdout(&run_install(&["--skills", "all", "--dir", &target], ""));
    assert_eq!(text.matches("is up to date").count(), ALL_SKILLS.len());
    assert!(!text.contains("Start a new agent session"));
}

#[test]
fn test_local_change_needs_force() {
    let folder = tempfile::tempdir().expect("the test creates a folder");
    let target = folder.path().to_string_lossy().to_string();
    let skill_file = folder.path().join("ste-code/SKILL.md");
    run_install(&["--skills", "ste-code", "--dir", &target], "");
    std::fs::write(&skill_file, "local edit\n").expect("the test edits the skill");
    run_install(&["--skills", "ste-code", "--dir", &target], "");
    assert_eq!(std::fs::read_to_string(&skill_file).expect("the file exists"), "local edit\n");
    run_install(&["--skills", "ste-code", "--dir", &target, "--force"], "");
    assert_ne!(std::fs::read_to_string(&skill_file).expect("the file exists"), "local edit\n");
}

#[test]
fn test_dry_run_and_blocked_folder() {
    let folder = tempfile::tempdir().expect("the test creates a folder");
    let target = folder.path().to_string_lossy().to_string();
    write_files(folder.path(), &[("ste-review/notes.txt", "keep\n")]);
    let text = get_stdout(&run_install(&["--skills", "all", "--dir", &target, "--dry-run", "--force"], ""));
    assert_eq!(text.matches("Dry run: install").count(), ALL_SKILLS.len() - 1);
    assert!(text.contains("has no SKILL.md file"));
    assert!(folder.path().join("ste-review/notes.txt").is_file());
}

#[test]
fn test_bad_requests_fail() {
    let unknown = run_install(&["--skills", "ste-nothing", "--dir", "."], "");
    assert_eq!(unknown.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&unknown.stderr).contains("Unknown skill: ste-nothing"));
    let silent = run_install(&[], "");
    assert_eq!(silent.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&silent.stderr).contains("got no answer"));
}

#[test]
fn test_adopt_writes_each_file_once() {
    let folder = tempfile::tempdir().expect("the test creates a folder");
    let target = folder.path().to_string_lossy().to_string();
    std::fs::create_dir_all(folder.path().join(".git/hooks")).expect("the test creates a git folder");
    write_files(folder.path(), &[("app/main.go", "package main\n")]);
    let first = get_stdout(&run_ste(&["adopt", "--dir", &target, "--linters", "--hook"], ""));
    assert!(first.contains("Profile: go"));
    for name in ["AGENTS.md", "CLAUDE.md", "LEXICON.md", ".golangci.yml", ".git/hooks/commit-msg"] {
        assert!(folder.path().join(name).is_file(), "{name}");
    }
    let second = get_stdout(&run_ste(&["adopt", "--dir", &target, "--linters", "--hook"], ""));
    assert_eq!(second.matches("Kept").count(), 5);
}

#[test]
fn test_install_script_with_a_local_binary() {
    let output = run_script(&["--list"], &[("STE_BINARY", BINARY)]);
    assert!(output.status.success());
    assert!(get_stdout(&output).contains("ste-adopt"));
}

#[test]
fn test_install_script_downloads_and_verifies() {
    let release = tempfile::tempdir().expect("the test creates a folder");
    let target = String::from_utf8_lossy(
        &Command::new("bash")
            .args(["-c", "case $(uname -m) in arm64|aarch64) echo aarch64;; *) echo x86_64;; esac"])
            .output()
            .expect("bash runs the script")
            .stdout,
    )
    .trim()
    .to_string();
    let system = if cfg!(target_os = "macos") { "apple-darwin" } else { "unknown-linux-musl" };
    let archive = format!("ste-{target}-{system}.tar.gz");
    let script = format!("cp {BINARY} ste && tar -czf {archive} ste && (sha256sum {archive} 2>/dev/null || shasum -a 256 {archive}) > {archive}.sha256");
    assert!(Command::new("bash")
        .args(["-c", &script])
        .current_dir(release.path())
        .status()
        .expect("bash runs the script")
        .success());
    let install_folder = tempfile::tempdir().expect("the test creates a folder");
    let url = format!("file://{}", release.path().display());
    let environment =
        [("STE_DOWNLOAD_URL", url.as_str()), ("STE_INSTALL_DIR", install_folder.path().to_str().unwrap_or_default())];
    assert!(run_script(&["--binary-only"], &environment).status.success());
    assert!(Command::new(install_folder.path().join("ste"))
        .arg("--version")
        .status()
        .expect("the installed ste binary runs")
        .success());
    std::fs::write(release.path().join(format!("{archive}.sha256")), "0000  bad\n")
        .expect("the test edits the checksum");
    assert_eq!(run_script(&["--binary-only"], &environment).status.code(), Some(1));
}
