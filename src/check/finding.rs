use std::collections::{BTreeMap, BTreeSet};
use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

use crate::spec::Standard;

pub const ALL_RULES: &str = "*";
pub type Suppressions = BTreeMap<usize, BTreeSet<String>>;

static RULE_ID: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Z]\d+\.\d+").expect("the pattern is a valid regex"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    pub fn from_name(name: &str) -> Self {
        match name {
            "error" => Self::Error,
            "info" => Self::Info,
            _ => Self::Warning,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub path: String,
    pub line: usize,
    pub rule: String,
    pub message: String,
    pub severity: Option<Severity>,
    pub column: usize,
}

impl Finding {
    pub fn new(line: usize, rule: &str, message: String) -> Self {
        Self { path: String::new(), line, rule: rule.to_string(), message, severity: None, column: 1 }
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = Some(severity);
        self
    }
}

pub fn parse_ignore_ids(text: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut rest = text;
    while let Some(start) = rest.find("ste-ignore") {
        let after = &rest[start + "ste-ignore".len()..];
        rest = after;
        if after.starts_with("-file") {
            continue;
        }
        let mut cursor = after.trim_start_matches([':', ' ', '\t']);
        let before = found.len();
        while let Some(identifier) = RULE_ID.find(cursor) {
            found.insert(identifier.as_str().to_string());
            cursor = cursor[identifier.end()..].trim_start_matches([',', ' ', '\t']);
        }
        if found.len() == before {
            found.insert(ALL_RULES.to_string());
        }
    }
    found
}

pub fn is_file_ignored(text: &str) -> bool {
    text.contains("ste-ignore-file")
}

fn is_suppressed(finding: &Finding, suppressions: &Suppressions) -> bool {
    suppressions.get(&finding.line).is_some_and(|rules| rules.contains(ALL_RULES) || rules.contains(&finding.rule))
}

pub fn filter_findings(findings: Vec<Finding>, standard: &Standard, suppressions: &Suppressions) -> Vec<Finding> {
    findings
        .into_iter()
        .filter(|finding| !standard.disabled.contains(&finding.rule) && !is_suppressed(finding, suppressions))
        .map(|finding| {
            let severity = finding.severity.unwrap_or_else(|| {
                Severity::from_name(standard.severities.get(&finding.rule).map_or("warning", String::as_str))
            });
            finding.with_severity(severity)
        })
        .collect()
}

pub fn sort_findings(findings: &mut [Finding]) {
    findings.sort_by(|left, right| {
        (&left.path, left.line, left.severity, &left.rule).cmp(&(&right.path, right.line, right.severity, &right.rule))
    });
}
