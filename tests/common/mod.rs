#![allow(dead_code)]

use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

pub const BINARY: &str = env!("CARGO_BIN_EXE_ste");

pub fn get_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn run_ste(arguments: &[&str], stdin: &str) -> Output {
    let mut child = Command::new(BINARY)
        .args(arguments)
        .current_dir(get_root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the ste binary starts");
    child
        .stdin
        .take()
        .expect("the standard input is a pipe")
        .write_all(stdin.as_bytes())
        .expect("the test writes the input");
    child.wait_with_output().expect("the ste binary finishes")
}

pub fn read_findings(output: &Output) -> Vec<serde_json::Value> {
    serde_json::from_slice(&output.stdout).unwrap_or_default()
}

pub fn list_rules(findings: &[serde_json::Value]) -> BTreeSet<String> {
    findings.iter().filter_map(|finding| finding["rule"].as_str().map(ToString::to_string)).collect()
}

pub fn write_files(folder: &Path, files: &[(&str, &str)]) {
    for (name, text) in files {
        let path = folder.join(name);
        std::fs::create_dir_all(path.parent().expect("a file has a parent folder"))
            .expect("the test creates the folder");
        std::fs::write(path, text).expect("the test writes the file");
    }
}

pub fn get_stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}
