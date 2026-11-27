use std::collections::BTreeSet;
use std::sync::LazyLock;

use regex::Regex;

static SENTENCE_END: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[.!?]+["')\]]*"#).expect("the pattern is a valid regex"));
static WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Za-z0-9][\w'\u{2019}./:+-]*").expect("the pattern is a valid regex"));
static CONDITION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(?:if|when|before|after|unless|while|once|until)\b[^,]*,\s*([a-z']+)")
        .expect("the pattern is a valid regex")
});
static MARKER_PREFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z]+:\s*").expect("the pattern is a valid regex"));

#[derive(Debug, Clone)]
pub struct Sentence {
    pub text: String,
    pub line: usize,
    pub words: usize,
    pub is_instruction: bool,
}

pub fn count_words(text: &str) -> usize {
    WORD.find_iter(text).count()
}

fn find_first_word(text: &str) -> String {
    WORD.find(text)
        .map(|found| found.as_str().to_lowercase().trim_end_matches(['.', ':', ',']).to_string())
        .unwrap_or_default()
}

pub fn is_instruction(text: &str, imperatives: &BTreeSet<String>) -> bool {
    let stripped = MARKER_PREFIX.replace(text.trim(), "");
    if imperatives.contains(&find_first_word(&stripped)) {
        return true;
    }
    CONDITION.captures(&stripped).is_some_and(|condition| imperatives.contains(&condition[1].to_lowercase()))
}

fn is_boundary(text: &str, end: usize) -> bool {
    let rest = &text[end..];
    let trimmed = rest.trim_start();
    if trimmed.is_empty() {
        return true;
    }
    let has_space = trimmed.len() < rest.len();
    let next = trimmed.chars().next().unwrap_or(' ');
    has_space && (next.is_ascii_uppercase() || next.is_ascii_digit() || "\"'([".contains(next))
}

fn is_abbreviation_end(text: &str, dot: usize, abbreviations: &BTreeSet<String>) -> bool {
    let word_start = text[..dot].rfind([' ', '\n']).map_or(0, |index| index + 1);
    abbreviations.contains(&text[word_start..=dot].to_lowercase())
}

fn list_spans(text: &str, abbreviations: &BTreeSet<String>) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = 0;
    for ending in SENTENCE_END.find_iter(text) {
        if !is_boundary(text, ending.end()) || is_abbreviation_end(text, ending.start(), abbreviations) {
            continue;
        }
        spans.push((start, ending.end()));
        start = ending.end();
    }
    if !text[start..].trim().is_empty() {
        spans.push((start, text.len()));
    }
    spans
}

pub struct SentenceWords<'a> {
    pub abbreviations: &'a BTreeSet<String>,
    pub imperatives: &'a BTreeSet<String>,
}

pub fn split(text: &str, line_numbers: &[usize], words: &SentenceWords) -> Vec<Sentence> {
    list_spans(text, words.abbreviations)
        .into_iter()
        .filter_map(|(start, end)| {
            let segment = &text[start..end];
            let leading = segment.len() - segment.trim_start().len();
            let row = text[..start + leading].matches('\n').count();
            let count = count_words(segment);
            let line = line_numbers[row.min(line_numbers.len().saturating_sub(1))];
            let sentence = segment.trim().to_string();
            (count > 0).then(|| Sentence {
                is_instruction: is_instruction(&sentence, words.imperatives),
                text: sentence,
                line,
                words: count,
            })
        })
        .collect()
}
