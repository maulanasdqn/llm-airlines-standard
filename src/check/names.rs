use std::sync::LazyLock;

use regex::Regex;

use super::lexer::Language;
use super::script_names;
use super::source::compute_depths;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameKind {
    Function,
    Type,
    Variable,
}

#[derive(Debug, Clone)]
pub struct Name {
    pub name: String,
    pub kind: NameKind,
    pub offset: usize,
    pub is_noun_allowed: bool,
}

impl Name {
    pub fn new(found: &regex::Match, kind: NameKind, is_noun_allowed: bool) -> Self {
        Self { name: found.as_str().to_string(), kind, offset: found.start(), is_noun_allowed }
    }
}

static PYTHON_DEF: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^[ \t]*(?:async\s+)?def\s+(\w+)").expect("the pattern is a valid regex"));
static PYTHON_CLASS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^[ \t]*class\s+(\w+)").expect("the pattern is a valid regex"));
static PYTHON_ASSIGN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^[ \t]*([A-Za-z_]\w*)\s*(?::[^=\n]+)?=([^=]|$)").expect("the pattern is a valid regex")
});
static PROPERTY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^@(property|cached_property|\w+\.(setter|getter|deleter))\b").expect("the pattern is a valid regex")
});
static GO_FUNCTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^func\s*(\([^)]*\))?\s*([A-Za-z_]\w*)\s*(?:\[[^\]]*\])?\s*\(([^)]*)\)")
        .expect("the pattern is a valid regex")
});
static GO_TYPE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*type\s+([A-Za-z_]\w*)").expect("the pattern is a valid regex"));
static GO_SHORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^\s*((?:[A-Za-z_]\w*\s*,\s*)*[A-Za-z_]\w*)\s*:=").expect("the pattern is a valid regex")
});
static GO_VAR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*(?:var|const)\s+([A-Za-z_]\w*)").expect("the pattern is a valid regex"));
static IDENTIFIER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Za-z_]\w*").expect("the pattern is a valid regex"));
static RUST_FN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bfn\s+([A-Za-z_]\w*)\s*(?:<[^>{]*>)?\s*\(([^)]*)\)").expect("the pattern is a valid regex")
});
static RUST_TYPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:struct|enum|trait|type|union)\s+([A-Za-z_]\w*)").expect("the pattern is a valid regex")
});
static RUST_LET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\blet\s+(?:mut\s+)?([A-Za-z_]\w*)").expect("the pattern is a valid regex"));
static RUST_CONST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:const|static)\s+(?:mut\s+)?([A-Za-z_]\w*)\s*:").expect("the pattern is a valid regex")
});
const SELF_PARAMETERS: [&str; 4] = ["self", "&self", "&mut self", "mut self"];

pub fn find(code_text: &str, language: Language) -> Vec<Name> {
    let names = match language {
        Language::Python => find_python_names(code_text),
        Language::Script => script_names::find(code_text),
        Language::Go => find_go_names(code_text),
        Language::Rust => find_rust_names(code_text),
    };
    names.into_iter().filter(|name| name.name != "_").collect()
}

fn is_property(code_text: &str, offset: usize) -> bool {
    let line_start = code_text[..offset].rfind('\n').unwrap_or_default();
    let previous = code_text[..line_start].trim_end().rsplit('\n').next().unwrap_or_default().trim();
    PROPERTY.is_match(previous)
}

fn find_python_names(code_text: &str) -> Vec<Name> {
    let mut names = Vec::new();
    for found in PYTHON_DEF.captures_iter(code_text).filter_map(|captures| captures.get(1)) {
        let text = found.as_str();
        let is_dunder = text.starts_with("__") && text.ends_with("__");
        names.push(Name::new(&found, NameKind::Function, is_dunder || is_property(code_text, found.start())));
    }
    names.extend(
        PYTHON_CLASS
            .captures_iter(code_text)
            .filter_map(|captures| captures.get(1))
            .map(|found| Name::new(&found, NameKind::Type, false)),
    );
    let depths = compute_depths(code_text);
    let assignments = PYTHON_ASSIGN.captures_iter(code_text).filter_map(|captures| captures.get(1));
    names.extend(
        assignments
            .filter(|found| depths[found.start()] == 0)
            .map(|found| Name::new(&found, NameKind::Variable, false)),
    );
    names
}

fn find_go_names(code_text: &str) -> Vec<Name> {
    let mut names = Vec::new();
    for captures in GO_FUNCTION.captures_iter(code_text) {
        let is_getter = captures.get(1).is_some() && captures[3].trim().is_empty();
        names.extend(captures.get(2).map(|found| Name::new(&found, NameKind::Function, is_getter)));
    }
    names.extend(
        GO_TYPE
            .captures_iter(code_text)
            .filter_map(|captures| captures.get(1))
            .map(|found| Name::new(&found, NameKind::Type, false)),
    );
    for list in GO_SHORT.captures_iter(code_text).filter_map(|captures| captures.get(1)) {
        for part in IDENTIFIER.find_iter(list.as_str()) {
            let offset = list.start() + part.start();
            names.push(Name {
                name: part.as_str().to_string(),
                kind: NameKind::Variable,
                offset,
                is_noun_allowed: false,
            });
        }
    }
    names.extend(
        GO_VAR
            .captures_iter(code_text)
            .filter_map(|captures| captures.get(1))
            .map(|found| Name::new(&found, NameKind::Variable, false)),
    );
    names
}

fn find_rust_names(code_text: &str) -> Vec<Name> {
    let mut names = Vec::new();
    for captures in RUST_FN.captures_iter(code_text) {
        let is_getter = SELF_PARAMETERS.contains(&captures[2].trim().trim_end_matches(','));
        names.extend(captures.get(1).map(|found| Name::new(&found, NameKind::Function, is_getter)));
    }
    let simple = [(&*RUST_TYPE, NameKind::Type), (&*RUST_LET, NameKind::Variable), (&*RUST_CONST, NameKind::Variable)];
    for (pattern, kind) in simple {
        names.extend(
            pattern
                .captures_iter(code_text)
                .filter_map(|captures| captures.get(1))
                .map(|found| Name::new(&found, kind, false)),
        );
    }
    names
}
