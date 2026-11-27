use std::collections::BTreeMap;
use std::sync::LazyLock;

use regex::Regex;

use crate::error::{Error, Result};

pub type Limits = BTreeMap<String, BTreeMap<String, u64>>;

static SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^## ([WC])(\d) (.+)$").expect("the pattern is a valid regex"));
static RULE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^### ([WC]\d\.\d+) (.+)$").expect("the pattern is a valid regex"));
static FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^- (Severity|Checks|Limit|Why|Do|Do not): (.+)$").expect("the pattern is a valid regex")
});
static LIMIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+)\.([a-z_]+) = (\d+)$").expect("the pattern is a valid regex"));

const SEVERITIES: [&str; 3] = ["error", "warning", "info"];
const TOOLS: [&str; 8] = ["review", "ste", "vale", "eslint", "ruff", "golangci", "revive", "clippy"];
const REQUIRED_FIELDS: [&str; 5] = ["Severity", "Checks", "Why", "Do", "Do not"];

#[derive(Debug, Clone, Default)]
pub struct Rule {
    pub id: String,
    pub title: String,
    pub section: u32,
    pub section_title: String,
    pub severity: String,
    pub checks: Vec<String>,
    pub fields: BTreeMap<String, String>,
    pub limits: Vec<(String, String, u64)>,
}

fn read_field(rule: &mut Rule, field: (&str, &str), source: &str) -> Result<()> {
    let (key, value) = field;
    if key != "Limit" {
        rule.fields.insert(key.to_string(), value.to_string());
        return Ok(());
    }
    let found = LIMIT.captures(value).ok_or_else(|| {
        Error::new(format!("{source}: rule {} has a bad limit \"{value}\". Write \"group.key = number\".", rule.id))
    })?;
    let number = found[3].parse::<u64>().unwrap_or_default();
    rule.limits.push((found[1].to_string(), found[2].to_string(), number));
    Ok(())
}

fn parse_lines(text: &str, source: &str) -> Result<Vec<Rule>> {
    let mut rules: Vec<Rule> = Vec::new();
    let mut section = (0, String::new());
    for line in text.lines() {
        if let Some(found) = SECTION.captures(line) {
            section = (found[2].parse().unwrap_or_default(), found[3].to_string());
        } else if let Some(found) = RULE.captures(line) {
            let (number, title) = &section;
            rules.push(Rule {
                id: found[1].to_string(),
                title: found[2].to_string(),
                section: *number,
                section_title: title.clone(),
                ..Rule::default()
            });
        } else if let (Some(found), Some(rule)) = (FIELD.captures(line), rules.last_mut()) {
            read_field(rule, (&found[1], &found[2]), source)?;
        }
    }
    Ok(rules)
}

fn validate_rule(rule: &Rule, source: &str) -> Result<()> {
    let missing: Vec<&str> = REQUIRED_FIELDS.into_iter().filter(|key| !rule.fields.contains_key(*key)).collect();
    if !missing.is_empty() {
        return Err(Error::new(format!("{source}: rule {} has no {} field.", rule.id, missing.join(", "))));
    }
    if !SEVERITIES.contains(&rule.severity.as_str()) {
        return Err(Error::new(format!("{source}: rule {} has a bad severity. Use error, warning, or info.", rule.id)));
    }
    let prefix = format!("{}{}.", &rule.id[..1], rule.section);
    if !rule.id.starts_with(&prefix) {
        return Err(Error::new(format!("{source}: rule {} is not in section {}.", rule.id, rule.section)));
    }
    validate_details(rule, source)
}

fn validate_details(rule: &Rule, source: &str) -> Result<()> {
    let text = format!("{} {}", rule.title, rule.fields.get("Why").map_or("", String::as_str));
    if let Some((group, key, value)) = rule.limits.iter().find(|(_, _, value)| !text.contains(&value.to_string())) {
        return Err(Error::new(format!(
            "{source}: rule {} sets {group}.{key} = {value}, but its text does not say {value}.",
            rule.id
        )));
    }
    if let Some(check) = rule.checks.iter().find(|check| !TOOLS.contains(&check.split(':').next().unwrap_or_default()))
    {
        return Err(Error::new(format!("{source}: rule {} names the unknown tool in \"{check}\".", rule.id)));
    }
    Ok(())
}

fn normalize_rule(mut rule: Rule, source: &str) -> Result<Rule> {
    rule.severity = rule.fields.get("Severity").cloned().unwrap_or_default();
    let checks = rule.fields.get("Checks").cloned().unwrap_or_default();
    rule.checks = checks
        .split(',')
        .map(|check| check.trim().trim_matches('`').to_string())
        .filter(|check| !check.is_empty())
        .collect();
    validate_rule(&rule, source)?;
    Ok(rule)
}

pub fn parse(text: &str, source: &str) -> Result<Vec<Rule>> {
    let rules = parse_lines(text, source)?
        .into_iter()
        .map(|rule| normalize_rule(rule, source))
        .collect::<Result<Vec<Rule>>>()?;
    let mut seen = std::collections::BTreeSet::new();
    if let Some(duplicate) = rules.iter().find(|rule| !seen.insert(rule.id.clone())) {
        return Err(Error::new(format!("{source}: the rule ID {} occurs more than once.", duplicate.id)));
    }
    Ok(rules)
}

pub fn build_limits(rules: &[Rule]) -> Limits {
    let mut limits = Limits::new();
    for (group, key, value) in rules.iter().flat_map(|rule| rule.limits.iter()) {
        limits.entry(group.clone()).or_default().insert(key.clone(), *value);
    }
    limits
}
