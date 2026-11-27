mod common;

use std::collections::BTreeSet;

use common::{list_rules, read_findings, run_ste, write_files};

const FIXTURE_CONFIG: &str = "tests/fixtures/ste.json";
const CODE_RULES: [&str; 9] = ["C1.3", "C1.6", "C1.7", "C2.1", "C3.1", "C5.1", "C6.1", "C6.2", "C7.3"];
const BAD_PROSE: [&str; 10] = ["W1.4", "W3.1", "W3.2", "W3.3", "W4.3", "W6.1", "W6.3", "W7.1", "W7.2", "W8.2"];

fn run_check(kind: &str, target: &str, stdin: &str) -> (i32, Vec<serde_json::Value>) {
    let output = run_ste(&["check", kind, target, "--format", "json", "--config", FIXTURE_CONFIG], stdin);
    (output.status.code().unwrap_or_default(), read_findings(&output))
}

fn run_project(kind: &str, files: &[(&str, &str)]) -> Vec<serde_json::Value> {
    let folder = tempfile::tempdir().expect("the test creates a folder");
    write_files(folder.path(), &[(".ste.json", "{}")]);
    write_files(folder.path(), files);
    let root = folder.path().to_string_lossy().to_string();
    let config = folder.path().join(".ste.json").to_string_lossy().to_string();
    read_findings(&run_ste(&["check", kind, &root, "--format", "json", "--config", &config], ""))
}

fn build_set(rules: &[&str]) -> BTreeSet<String> {
    rules.iter().map(ToString::to_string).collect()
}

#[test]
fn test_bad_code_fixtures_break_expected_rules() {
    let cases: [(&str, Vec<&str>); 4] = [
        ("tests/fixtures/code/typescript/bad.ts", [&CODE_RULES[..], &["C4.4"]].concat()),
        ("tests/fixtures/code/python/bad.py", [&CODE_RULES[..], &["C4.4"]].concat()),
        ("tests/fixtures/code/go/bad", CODE_RULES.iter().copied().filter(|rule| *rule != "C6.1").collect()),
        ("tests/fixtures/code/rust/bad", CODE_RULES.to_vec()),
    ];
    for (fixture, expected) in cases {
        let (code, findings) = run_check("code", fixture, "");
        assert_eq!(code, 1, "{fixture}");
        assert!(build_set(&expected).is_subset(&list_rules(&findings)), "{fixture}: {:?}", list_rules(&findings));
    }
}

#[test]
fn test_good_code_fixtures_have_no_findings() {
    for fixture in ["typescript/good.ts", "python/good.py", "go/good", "rust/good"] {
        let (code, findings) = run_check("code", &format!("tests/fixtures/code/{fixture}"), "");
        assert_eq!((code, findings.len()), (0, 0), "{fixture}: {findings:?}");
    }
}

#[test]
fn test_bad_prose_breaks_expected_rules() {
    let (code, findings) = run_check("prose", "tests/fixtures/prose/bad.md", "");
    assert_eq!(code, 1);
    assert!(build_set(&BAD_PROSE).is_subset(&list_rules(&findings)));
}

#[test]
fn test_good_prose_has_no_findings() {
    assert_eq!(run_check("prose", "tests/fixtures/prose/good.md", "").1.len(), 0);
}

#[test]
fn test_bad_commit_from_stdin() {
    let (code, findings) = run_check("commit", "-", "Fixed stuff.\n");
    assert_eq!(code, 1);
    assert_eq!(findings.len(), 3);
    assert_eq!(list_rules(&findings), build_set(&["C9.4"]));
}

#[test]
fn test_good_commit_has_no_findings() {
    assert_eq!(run_check("commit", "tests/fixtures/commit/good.txt", "").1.len(), 0);
}

#[test]
fn test_subject_without_blank_line() {
    let findings = run_check("commit", "-", "Add the retry limit\nThe body starts here.\n").1;
    assert!(findings.iter().any(|finding| finding["line"] == 2));
}

#[test]
fn test_file_length_limit() {
    let findings = run_project("code", &[("long.py", &"total = 1\n".repeat(201))]);
    assert_eq!(list_rules(&findings), build_set(&["C5.7"]));
}

#[test]
fn test_code_suppression() {
    let source = "# ste-ignore: C1.6\ndef process_order() -> None:\n    return None\n";
    assert!(run_project("code", &[("order.py", source)]).is_empty());
}

#[test]
fn test_markdown_suppression() {
    assert!(run_project("prose", &[("notes.md", "<!-- ste-ignore W1.4 -->\nLeverage the cache.\n")]).is_empty());
}

#[test]
fn test_lexicon_adds_technical_verbs() {
    let source = "def reconcile_ledger() -> None:\n    return None\n";
    let lexicon = "## Technical verbs\n\n| Word | Meaning |\n|---|---|\n| `reconcile` | Match two records. |\n";
    assert_eq!(list_rules(&run_project("code", &[("ledger.py", source)])), build_set(&["C3.1"]));
    assert!(run_project("code", &[("ledger.py", source), ("LEXICON.md", lexicon)]).is_empty());
}

#[test]
fn test_directives_are_allowed() {
    let source = "#!/usr/bin/env python3\nTOTAL = 1  # noqa: E501\nNAMES = []  # type: ignore[var-annotated]\n";
    assert!(run_project("code", &[("tool.py", source)]).is_empty());
}

#[test]
fn test_rust_literals_hide_comment_markers() {
    let source = "pub const QUOTE: char = '\\'';\npub const PATH: &str = r#\"// not a comment\"#;\npub fn get_limit<'a>(text: &'a str) -> &'a str {\n    text\n}\n";
    assert!(run_project("code", &[("lib.rs", source)]).is_empty());
}

#[test]
fn test_script_regex_literals_hide_comment_markers() {
    let source = "export const BLOCK_COMMENT = /\\/\\*[\\s\\S]*?\\*\\//g;\nexport const MARKERS = /[/*]/g;\n";
    assert!(run_project("code", &[("markers.ts", source)]).is_empty());
}

#[test]
fn test_script_division_keeps_comment_markers() {
    let source = "export const getHalf = (total: number): number => total / 2; // the half\n";
    assert_eq!(list_rules(&run_project("code", &[("half.ts", source)])), build_set(&["C6.1"]));
}

#[test]
fn test_python_format_strings_are_messages() {
    let source = "def load_order(order_id: str) -> None:\n    raise ValueError(f\"bad\")\n";
    assert_eq!(list_rules(&run_project("code", &[("order.py", source)])), build_set(&["C7.3"]));
}

#[test]
fn test_split_words() {
    let split = |name: &str| ste::check::words::split(name);
    assert_eq!(split("parseHTTPResponse"), ["parse", "HTTP", "Response"]);
    assert_eq!(split("MAX_RETRY_COUNT"), ["MAX", "RETRY", "COUNT"]);
    assert_eq!(split("_load_config"), ["load", "config"]);
    assert_eq!(split("userID"), ["user", "ID"]);
}
