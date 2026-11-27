use std::collections::{BTreeMap, BTreeSet};

use super::finding::{Finding, Severity};
use super::vocabulary::Vocabulary;

const TOP_NOUNS: usize = 25;
const MIN_ROTATION_VERBS: usize = 2;

#[derive(Debug, Clone)]
pub struct NameUse {
    pub name: String,
    pub words: Vec<String>,
    pub path: String,
    pub line: usize,
    pub language: String,
}

type RotationIndex<'a> = BTreeMap<(String, String), BTreeMap<String, Vec<&'a NameUse>>>;

fn build_index(uses: &[NameUse]) -> RotationIndex<'_> {
    let mut index = RotationIndex::new();
    for name_use in uses.iter().filter(|name_use| name_use.words.len() > 1) {
        let object = (name_use.language.clone(), name_use.words[1..].join(" "));
        index.entry(object).or_default().entry(name_use.words[0].clone()).or_default().push(name_use);
    }
    index
}

fn find_group_rotation(
    verbs: &BTreeMap<String, Vec<&NameUse>>,
    group: &[String],
    vocabulary: &Vocabulary,
) -> Vec<Finding> {
    let used: Vec<&String> = group.iter().filter(|verb| verbs.contains_key(*verb)).collect();
    if used.len() < MIN_ROTATION_VERBS {
        return Vec::new();
    }
    let is_approved = used.iter().all(|verb| vocabulary.verbs.contains(*verb));
    let severity = if is_approved { Severity::Info } else { Severity::Warning };
    let names: BTreeSet<&str> = used.iter().map(|verb| verbs[*verb][0].name.as_str()).collect();
    let listed = names.into_iter().collect::<Vec<&str>>().join(", ");
    let message = format!(
        "These names use {} verbs for one object: {listed}. Use one verb, or keep each verb contract (C3.2).",
        used.len()
    );
    used[1..]
        .iter()
        .flat_map(|verb| verbs[*verb].iter())
        .map(|name_use| {
            Finding::new(name_use.line, "C1.3", message.clone()).with_path(&name_use.path).with_severity(severity)
        })
        .collect()
}

pub fn find_rotations(uses: &[NameUse], vocabulary: &Vocabulary, groups: &[Vec<String>]) -> Vec<Finding> {
    let index = build_index(uses);
    index
        .values()
        .flat_map(|verbs| groups.iter().flat_map(|group| find_group_rotation(verbs, group, vocabulary)))
        .collect()
}

fn count_uses<'a>(words: impl Iterator<Item = &'a String>) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for word in words {
        *counts.entry(word.clone()).or_default() += 1;
    }
    let mut sorted: Vec<(String, usize)> = counts.into_iter().collect();
    sorted.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    sorted
}

fn render_table(title: &str, rows: &[(String, usize)]) -> Vec<String> {
    let mut lines = vec![
        format!("## {title}"),
        String::new(),
        "| Word | Meaning | Uses |".to_string(),
        "|---|---|---|".to_string(),
    ];
    lines.extend(rows.iter().map(|(word, count)| format!("| `{word}` | (write one meaning) | {count} |")));
    lines.push(String::new());
    lines
}

fn render_findings(uses: &[NameUse], vocabulary: &Vocabulary) -> Vec<String> {
    let is_avoided =
        |word: &&String| vocabulary.avoid_verbs.contains_key(*word) || vocabulary.avoid_nouns.contains_key(*word);
    let avoided = count_uses(uses.iter().flat_map(|name_use| name_use.words.iter()).filter(is_avoided));
    let mut lines = vec![
        "## Findings".to_string(),
        String::new(),
        "These words break C1.6. Replace them in the code.".to_string(),
        String::new(),
    ];
    for (word, count) in avoided {
        let replacement = vocabulary
            .avoid_verbs
            .get(&word)
            .or_else(|| vocabulary.avoid_nouns.get(&word))
            .cloned()
            .unwrap_or_default();
        lines.push(format!("- `{word}` ({count} uses): use {replacement}."));
    }
    lines.push(String::new());
    lines
}

pub fn render_lexicon(uses: &[NameUse], vocabulary: &Vocabulary) -> String {
    let is_known = |word: &String| {
        vocabulary.verbs.contains(word)
            || vocabulary.predicates.contains(word)
            || vocabulary.avoid_verbs.contains_key(word)
    };
    let mut verbs =
        count_uses(uses.iter().filter_map(|name_use| name_use.words.first()).filter(|word| !is_known(word)));
    verbs.sort();
    let nouns = count_uses(
        uses.iter()
            .flat_map(|name_use| name_use.words.iter().skip(1))
            .filter(|word| !vocabulary.avoid_nouns.contains_key(*word)),
    );
    let mut lines = vec![
        "# Lexicon draft".to_string(),
        String::new(),
        "Review each row. Write one meaning for each word, or delete the row.".to_string(),
        String::new(),
    ];
    lines.extend(render_table("Technical verbs", &verbs));
    lines.extend(render_table("Technical nouns", &nouns.into_iter().take(TOP_NOUNS).collect::<Vec<_>>()));
    lines.extend(render_findings(uses, vocabulary));
    lines.join("\n")
}
