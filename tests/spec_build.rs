mod common;

use std::collections::BTreeSet;

use common::{get_root, run_ste};
use ste::spec::load_embedded_rules;
use ste::spec::rules::build_limits;

#[test]
fn test_generated_files_are_current() {
    let root = get_root().to_string_lossy().to_string();
    let output = run_ste(&["build", "--check", "--root", &root], "");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_both_rule_sets_cover_nine_sections() {
    let rules = load_embedded_rules().expect("the embedded rules parse");
    for prefix in ["W", "C"] {
        let sections: BTreeSet<u32> =
            rules.iter().filter(|rule| rule.id.starts_with(prefix)).map(|rule| rule.section).collect();
        assert_eq!(sections, (1..=9).collect::<BTreeSet<u32>>(), "{prefix}");
    }
}

#[test]
fn test_limits_from_the_rules() {
    let limits = build_limits(&load_embedded_rules().expect("the embedded rules parse"));
    assert_eq!(limits["code"]["file_lines"], 200);
    assert_eq!(limits["code"]["function_lines"], 40);
    assert_eq!(limits["prose"]["instruction_words"], 20);
}

#[test]
fn test_skills_are_valid() {
    assert!(ste::build::skills::validate_all(&get_root()).is_empty());
}

#[test]
fn test_generated_files_have_no_placeholders() {
    let root = get_root();
    for folder in ["templates/agents", "linters"] {
        for relative in ste::disk::list_tree(&root.join(folder)) {
            let is_instruction = !relative.contains('/') && (relative == "AGENTS.md" || relative == "CLAUDE.md");
            if is_instruction || relative.starts_with("src/") {
                continue;
            }
            let text = std::fs::read_to_string(root.join(folder).join(&relative)).expect("the test reads the file");
            assert!(!text.contains("{{"), "{folder}/{relative}");
        }
    }
}

#[test]
fn test_linters_take_limits_from_the_rules() {
    let root = get_root();
    let read = |path: &str| std::fs::read_to_string(root.join(path)).expect("the test reads the config");
    assert!(read("linters/python/ruff.toml").contains("max-positional-args = 3"));
    assert!(read("linters/typescript/eslint.config.mjs").contains("\"max-lines\": [\"error\", { max: 200"));
    assert!(read("linters/rust/clippy.toml").contains("excessive-nesting-threshold = 4"));
    assert!(read("Cargo.toml").contains(&read("linters/rust/lints.toml")));
}
