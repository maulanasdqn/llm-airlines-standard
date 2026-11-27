use std::sync::LazyLock;

use regex::Regex;

use super::blocks::{count_code_lines, measure_function};
use super::finding::{is_file_ignored, parse_ignore_ids, Finding, Suppressions};
use super::lexer::{Language, Token, TokenKind};
use super::name_rules::CodeContext;
use super::names::{Name, NameKind};
use super::source::{find_line, is_doc_comment, to_comment_body};

static ERROR_CALL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:new\s+\w*Error|raise\s+[\w.]+|errors\.New|fmt\.Errorf|panic!?|bail!|anyhow!|\.expect)\s*\(\s*$")
        .expect("the pattern is a valid regex")
});
static GO_DECLARATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?:func|type|var|const|package)\b").expect("the pattern is a valid regex"));
static LETTERS: LazyLock<Regex> = LazyLock::new(|| Regex::new("[A-Za-z]+").expect("the pattern is a valid regex"));
const COMMENT_MESSAGE: &str =
    "Remove this comment. Put the meaning in a name, a constant, or a type. Put the reason in the commit message.";
const DOC_MESSAGE: &str = "Remove this doc comment. The name, the parameters, and the types describe the code.";
const DOCSTRING_MESSAGE: &str = "Remove this docstring. The name, the parameters, and the types describe the code.";
const MIN_MESSAGE_WORDS: usize = 3;
const MESSAGE_LOOKBACK: usize = 80;

pub struct SourceView {
    pub language: Language,
    pub text: String,
    pub code_text: String,
    pub tokens: Vec<Token>,
    pub docstrings: Vec<Token>,
    pub line_index: Vec<usize>,
}

impl SourceView {
    pub fn get_token_text(&self, token: &Token) -> &str {
        &self.text[token.start..token.end.min(self.text.len())]
    }

    pub fn get_line_number(&self, offset: usize) -> usize {
        find_line(&self.line_index, offset)
    }
}

fn is_directive(view: &SourceView, token: &Token, directives: &[Regex]) -> bool {
    let text = view.get_token_text(token);
    if text.starts_with("#!") {
        return token.start == 0;
    }
    directives.iter().any(|directive| directive.is_match(to_comment_body(text)))
}

fn is_doc(view: &SourceView, token: &Token) -> bool {
    if view.language == Language::Go {
        return GO_DECLARATION.is_match(&view.code_text[token.end.min(view.code_text.len())..]);
    }
    is_doc_comment(view.get_token_text(token), view.language)
}

pub fn find_comment_findings(view: &SourceView, context: &CodeContext) -> Vec<Finding> {
    let directives: Vec<Regex> =
        context.standard.dictionary.directives.iter().filter_map(|pattern| Regex::new(pattern).ok()).collect();
    let mut findings: Vec<Finding> = view
        .tokens
        .iter()
        .filter(|token| token.kind == TokenKind::Comment && !is_directive(view, token, &directives))
        .map(|token| {
            let (rule, message) = if is_doc(view, token) { ("C6.2", DOC_MESSAGE) } else { ("C6.1", COMMENT_MESSAGE) };
            Finding::new(view.get_line_number(token.start), rule, message.to_string())
        })
        .collect();
    findings.extend(
        view.docstrings
            .iter()
            .map(|token| Finding::new(view.get_line_number(token.start), "C6.2", DOCSTRING_MESSAGE.to_string())),
    );
    findings
}

pub fn find_file_length(view: &SourceView, context: &CodeContext) -> Vec<Finding> {
    let limit = context.standard.get_limit("code", "file_lines");
    let count = count_code_lines(&view.text);
    if count <= limit {
        return Vec::new();
    }
    vec![Finding::new(
        1,
        "C5.7",
        format!("This file has {count} lines of code. The limit is {limit}. Split it by topic."),
    )]
}

pub fn find_function_lengths(view: &SourceView, names: &[Name], context: &CodeContext) -> Vec<Finding> {
    let limit = context.standard.get_limit("code", "function_lines");
    names
        .iter()
        .filter(|name| name.kind == NameKind::Function)
        .filter_map(|name| {
            let length = measure_function(&view.code_text, name, view.language);
            let message =
                format!("\"{}\" has {length} lines. The limit is {limit}. Split it into named steps.", name.name);
            (length > limit).then(|| Finding::new(view.get_line_number(name.offset), "C5.1", message))
        })
        .collect()
}

fn strip_quotes(text: &str) -> &str {
    text.trim_start_matches(['r', 'R', 'b', 'B', 'u', 'U', 'f', 'F']).trim_matches('#').trim_matches(['"', '\'', '`'])
}

fn is_vague_message(message: &str, vague: &[String]) -> bool {
    let normalized = message.trim().trim_end_matches(['.', '!']).to_lowercase();
    LETTERS.find_iter(message).count() < MIN_MESSAGE_WORDS || vague.contains(&normalized)
}

fn find_char_start(text: &str, offset: usize) -> usize {
    (0..=offset).rev().find(|position| text.is_char_boundary(*position)).unwrap_or_default()
}

pub fn find_error_messages(view: &SourceView, context: &CodeContext) -> Vec<Finding> {
    let vague = &context.standard.dictionary.vague_messages;
    let is_error_call = |token: &Token| {
        let start = find_char_start(&view.code_text, token.start.saturating_sub(MESSAGE_LOOKBACK));
        ERROR_CALL.is_match(&view.code_text[start..token.start])
    };
    view.tokens
        .iter()
        .filter(|token| token.kind == TokenKind::Literal && is_error_call(token))
        .filter_map(|token| {
            let message = strip_quotes(view.get_token_text(token));
            let text = format!("The error message \"{message}\" is vague. Say what failed, why, and what to do next.");
            is_vague_message(message, vague).then(|| Finding::new(view.get_line_number(token.start), "C7.3", text))
        })
        .collect()
}

pub fn find_suppressions(view: &SourceView) -> Suppressions {
    let mut suppressions = Suppressions::new();
    for token in view.tokens.iter().filter(|token| token.kind == TokenKind::Comment) {
        let rules = parse_ignore_ids(view.get_token_text(token));
        if rules.is_empty() {
            continue;
        }
        let line = view.get_line_number(token.start);
        let line_start = view.text[..token.start].rfind('\n').map_or(0, |index| index + 1);
        let is_alone = view.text[line_start..token.start].trim().is_empty();
        suppressions.entry(if is_alone { line + 1 } else { line }).or_default().extend(rules);
    }
    suppressions
}

pub fn is_view_ignored(view: &SourceView) -> bool {
    view.tokens.iter().any(|token| token.kind == TokenKind::Comment && is_file_ignored(view.get_token_text(token)))
}
