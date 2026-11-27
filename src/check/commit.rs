use std::collections::BTreeSet;
use std::io::Read;
use std::sync::LazyLock;

use regex::Regex;

use super::finding::{filter_findings, Finding, Severity, Suppressions};
use super::prose::find_text_findings;
use super::prose_rules::ProseContext;
use crate::disk::read_text;
use crate::error::{Error, Result};
use crate::spec::Standard;

static CONVENTIONAL_PREFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z]+(?:\([^)]*\))?!?:\s+").expect("the pattern is a valid regex"));
static TRAILER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z][\w-]*: \S").expect("the pattern is a valid regex"));
static WORDS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").expect("the pattern is a valid regex"));
const SCISSORS: &str = "# ------------------------ >8 ------------------------";
const VAGUE_WORDS: [&str; 11] =
    ["stuff", "things", "misc", "various", "wip", "tweaks", "changes", "fixes", "updates", "minor", "cleanup"];
const WEAK_WORDS: [&str; 8] = ["update", "fix", "bug", "bugs", "some", "small", "the", "a"];
const PAST_FORMS: [&str; 33] = [
    "made", "wrote", "ran", "built", "sent", "took", "gave", "found", "kept", "left", "brought", "told", "chose",
    "broke", "froze", "hid", "got", "began", "drew", "threw", "lost", "paid", "said", "spent", "led", "met", "bound",
    "stuck", "did", "went", "undid", "rewrote", "fed",
];
const COMMIT_VERBS: [&str; 31] = [
    "fix",
    "bump",
    "refactor",
    "revert",
    "drop",
    "document",
    "improve",
    "simplify",
    "support",
    "upgrade",
    "downgrade",
    "prepare",
    "release",
    "tidy",
    "clean",
    "speed",
    "tune",
    "pin",
    "unpin",
    "port",
    "extract",
    "inline",
    "bring",
    "handle",
    "process",
    "change",
    "rename",
    "add",
    "update",
    "remove",
    "delete",
];

fn read_message(source: &str) -> Result<String> {
    if source != "-" {
        return read_text(std::path::Path::new(source));
    }
    let mut text = String::new();
    std::io::stdin()
        .read_to_string(&mut text)
        .map_err(|error| Error::new(format!("Cannot read the standard input: {error}.")))?;
    Ok(text)
}

fn strip_git_comments(text: &str) -> Vec<String> {
    let lines = text.lines().take_while(|line| !line.starts_with(SCISSORS)).filter(|line| !line.starts_with('#'));
    lines.map(|line| line.trim_end().to_string()).skip_while(|line| line.trim().is_empty()).collect()
}

fn find_inflection(word: &str, verbs: &BTreeSet<String>) -> Option<&'static str> {
    if verbs.contains(word) {
        return None;
    }
    let is_verb = |stem: &str| verbs.contains(stem);
    let stem = |count: usize| word.get(..word.len().saturating_sub(count)).unwrap_or_default();
    let is_past = word.ends_with("ed")
        && (is_verb(stem(1)) || is_verb(stem(2)) || is_verb(stem(3)) || is_verb(&format!("{}y", stem(3))));
    if PAST_FORMS.contains(&word) || is_past {
        return Some("past");
    }
    if word.ends_with("ing") && (is_verb(stem(3)) || is_verb(&format!("{}e", stem(3))) || is_verb(stem(4))) {
        return Some("-ing");
    }
    let is_third = word.ends_with('s') && (is_verb(stem(1)) || is_verb(stem(2)) || is_verb(&format!("{}y", stem(3))));
    is_third.then_some("third-person")
}

fn find_subject_wording(subject: &str, verbs: &BTreeSet<String>) -> Vec<(Severity, String)> {
    let lowered = subject.to_lowercase();
    let words: Vec<&str> = WORDS.find_iter(&lowered).map(|found| found.as_str()).collect();
    let Some(first) = words.first() else {
        return Vec::new();
    };
    let mut results = Vec::new();
    if let Some(form) = find_inflection(first, verbs) {
        results.push((
            Severity::Error,
            format!("\"{first}\" is a {form} form. Write the subject in the imperative, such as \"Fix\" or \"Add\"."),
        ));
    }
    let has_vague = words.iter().any(|word| VAGUE_WORDS.contains(word));
    let is_weak = words.iter().all(|word| WEAK_WORDS.contains(word) || VAGUE_WORDS.contains(word));
    if has_vague || is_weak {
        results.push((Severity::Warning, String::from("The subject is vague. Say what changed and where.")));
    }
    results
}

fn find_subject_findings(subject: &str, standard: &Standard, verbs: &BTreeSet<String>) -> Vec<Finding> {
    let soft = standard.get_limit("commit", "subject_chars");
    let hard = standard.get_limit("commit", "subject_hard_chars");
    let length = subject.chars().count();
    let mut results = Vec::new();
    if length > hard {
        results.push((Severity::Error, format!("The subject has {length} characters. The limit is {hard}.")));
    } else if length > soft {
        results.push((Severity::Warning, format!("The subject has {length} characters. Keep it to {soft} or fewer.")));
    }
    if subject.ends_with('.') {
        results.push((Severity::Error, String::from("Do not end the subject with a period.")));
    }
    results.extend(find_subject_wording(&CONVENTIONAL_PREFIX.replace(subject, ""), verbs));
    results.into_iter().map(|(severity, message)| Finding::new(1, "C9.4", message).with_severity(severity)).collect()
}

fn find_body_layout(lines: &[String], standard: &Standard) -> Vec<Finding> {
    let mut findings = Vec::new();
    if lines.get(1).is_some_and(|line| !line.trim().is_empty()) {
        findings.push(
            Finding::new(2, "C9.4", String::from("Put a blank line after the subject.")).with_severity(Severity::Error),
        );
    }
    let limit = standard.get_limit("commit", "body_line_chars");
    let long_lines =
        lines.iter().enumerate().skip(2).filter(|(_, line)| line.chars().count() > limit && !line.contains("://"));
    for (index, line) in long_lines {
        let message = format!("This body line has {} characters. Wrap the body at {limit}.", line.chars().count());
        findings.push(Finding::new(index + 1, "C9.4", message).with_severity(Severity::Warning));
    }
    findings
}

fn build_verbs(standard: &Standard) -> BTreeSet<String> {
    let dictionary = &standard.dictionary;
    let mut verbs: BTreeSet<String> = dictionary.verbs.iter().map(|entry| entry.word.clone()).collect();
    verbs.extend(dictionary.prose.imperatives.iter().cloned());
    verbs.extend(COMMIT_VERBS.iter().map(ToString::to_string));
    verbs
}

fn find_body_prose(lines: &[String], standard: &Standard) -> Vec<Finding> {
    let body: Vec<&str> = ["", ""]
        .into_iter()
        .chain(lines.iter().skip(2).map(|line| if TRAILER.is_match(line) { "" } else { line.as_str() }))
        .collect();
    let (findings, suppressions) = find_text_findings(&body.join("\n"), &ProseContext::new(standard));
    filter_findings(findings, standard, &suppressions)
}

pub fn run(source: &str, standard: &Standard) -> Result<Vec<Finding>> {
    let label = if source == "-" { "<stdin>" } else { source };
    let lines = strip_git_comments(&read_message(source)?);
    let Some(subject) = lines.first() else {
        let empty = Finding::new(1, "C9.4", String::from("The commit message is empty. Write a subject line."))
            .with_severity(Severity::Error);
        return Ok(vec![empty.with_path(label)]);
    };
    let mut findings = find_subject_findings(subject, standard, &build_verbs(standard));
    findings.extend(find_body_layout(&lines, standard));
    findings.extend(find_body_prose(&lines, standard));
    let located = findings.into_iter().map(|finding| finding.with_path(label)).collect();
    Ok(filter_findings(located, standard, &Suppressions::new()))
}
