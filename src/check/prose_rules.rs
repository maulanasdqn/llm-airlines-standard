use std::collections::BTreeSet;
use std::sync::LazyLock;

use regex::Regex;

use super::finding::Finding;
use super::markdown::{Block, BlockKind};
use super::sentences::{is_instruction, Sentence};
use super::verb_forms::{find_passive, find_perfect, find_progressive, VerbWords};
use crate::spec::dictionary::ProseWords;
use crate::spec::Standard;

static JOINERS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(";|\u{2014}|\u{2013}| -- ").expect("the pattern is a valid regex"));
static MARKER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(WARNING|CAUTION|NOTE|Warning|Caution|Note|IMPORTANT|Important|DANGER|Danger|TIP|Tip)\s*:")
        .expect("the pattern is a valid regex")
});
static LETTERS: LazyLock<Regex> = LazyLock::new(|| Regex::new("[a-z]+").expect("the pattern is a valid regex"));
type FormCheck = fn(&str, &VerbWords) -> Option<String>;
const STANDARD_MARKERS: [&str; 3] = ["WARNING", "CAUTION", "NOTE"];
const COMMAND_MARKERS: [&str; 2] = ["WARNING", "CAUTION"];

pub struct ProseContext<'a> {
    pub standard: &'a Standard,
    pub prose: &'a ProseWords,
    pub verbs: VerbWords,
    pub hedges: BTreeSet<String>,
    pub imperatives: BTreeSet<String>,
    pub avoided: Vec<(Regex, String)>,
}

impl<'a> ProseContext<'a> {
    pub fn new(standard: &'a Standard) -> Self {
        let prose = &standard.dictionary.prose;
        let avoided = prose
            .avoid
            .iter()
            .filter_map(|entry| {
                let pattern = format!(r"(?i)(?:^|[^\w-]){}(?:$|[^\w-])", regex::escape(&entry.text));
                let message = format!("Do not use \"{}\". Instead, write: {}.", entry.text, entry.replacement);
                Regex::new(&pattern).ok().map(|regex| (regex, format!("{}\u{0}{message}", entry.rule)))
            })
            .collect();
        let collect = |words: &[String]| words.iter().cloned().collect::<BTreeSet<String>>();
        Self {
            standard,
            prose,
            verbs: VerbWords::from_prose(prose),
            hedges: collect(&prose.hedges),
            imperatives: collect(&prose.imperatives),
            avoided,
        }
    }
}

fn find_length(sentence: &Sentence, context: &ProseContext) -> Vec<Finding> {
    let (key, rule, kind) = if sentence.is_instruction {
        ("instruction_words", "W5.1", "This instruction")
    } else {
        ("description_words", "W6.1", "This sentence")
    };
    let limit = context.standard.get_limit("prose", key);
    if sentence.words <= limit {
        return Vec::new();
    }
    let message =
        format!("{kind} has {} words. The limit is {limit}. Split it into shorter sentences.", sentence.words);
    vec![Finding::new(sentence.line, rule, message)]
}

fn find_verb_forms(sentence: &Sentence, context: &ProseContext) -> Vec<Finding> {
    let text = sentence.text.to_lowercase();
    let checks: [(&str, FormCheck); 3] = [("W3.3", find_passive), ("W3.1", find_perfect), ("W3.2", find_progressive)];
    checks
        .into_iter()
        .filter_map(|(rule, check)| {
            check(&text, &context.verbs).map(|message| Finding::new(sentence.line, rule, message))
        })
        .collect()
}

fn find_hedges(sentence: &Sentence, context: &ProseContext) -> Vec<Finding> {
    let lowered = sentence.text.to_lowercase();
    let hedges: Vec<&str> =
        LETTERS.find_iter(&lowered).map(|found| found.as_str()).filter(|word| context.hedges.contains(*word)).collect();
    let limit = context.standard.get_limit("prose", "hedges_per_sentence");
    if hedges.len() <= limit {
        return Vec::new();
    }
    let message = format!(
        "This sentence has {} hedges ({}). Keep {limit} or fewer. State the fact.",
        hedges.len(),
        hedges.join(", ")
    );
    vec![Finding::new(sentence.line, "W4.3", message)]
}

fn find_avoided_words(sentence: &Sentence, context: &ProseContext) -> Vec<Finding> {
    context
        .avoided
        .iter()
        .filter(|(pattern, _)| pattern.is_match(&sentence.text))
        .filter_map(|(_, packed)| packed.split_once('\u{0}'))
        .map(|(rule, message)| Finding::new(sentence.line, rule, message.to_string()))
        .collect()
}

fn find_joiners(sentence: &Sentence, _context: &ProseContext) -> Vec<Finding> {
    if !JOINERS.is_match(&sentence.text) {
        return Vec::new();
    }
    vec![Finding::new(
        sentence.line,
        "W8.2",
        String::from("A semicolon or a dash joins two sentences. Write two sentences."),
    )]
}

pub fn find_sentence_findings(sentence: &Sentence, context: &ProseContext) -> Vec<Finding> {
    let rules: [fn(&Sentence, &ProseContext) -> Vec<Finding>; 5] =
        [find_length, find_verb_forms, find_hedges, find_avoided_words, find_joiners];
    rules.iter().flat_map(|rule| rule(sentence, context)).collect()
}

fn find_marker(block: &Block, first: &Sentence) -> String {
    if block.kind == BlockKind::Alert {
        return block.marker.clone();
    }
    MARKER.captures(&first.text).map(|found| found[1].to_string()).unwrap_or_default()
}

fn find_marker_findings(block: &Block, sentences: &[Sentence], context: &ProseContext) -> Vec<Finding> {
    let first = &sentences[0];
    let marker = find_marker(block, first);
    if marker.is_empty() {
        return Vec::new();
    }
    if !STANDARD_MARKERS.contains(&marker.as_str()) {
        let message = format!("Use WARNING, CAUTION, or NOTE in capitals. Found \"{marker}\".");
        return vec![Finding::new(first.line, "W7.1", message)];
    }
    if COMMAND_MARKERS.contains(&marker.as_str()) && !is_instruction(&first.text, &context.imperatives) {
        let message = format!("Start the {marker} with a clear command, such as \"Do not\" or \"Back up\".");
        return vec![Finding::new(first.line, "W7.2", message)];
    }
    Vec::new()
}

pub fn find_block_findings(block: &Block, sentences: &[Sentence], context: &ProseContext) -> Vec<Finding> {
    if sentences.is_empty() {
        return Vec::new();
    }
    let mut findings = find_marker_findings(block, sentences, context);
    let limit = context.standard.get_limit("prose", "paragraph_sentences");
    if sentences.len() > limit {
        let message =
            format!("This paragraph has {} sentences. The limit is {limit}. Split it by topic.", sentences.len());
        findings.push(Finding::new(sentences[0].line, "W6.3", message));
    }
    findings
}
