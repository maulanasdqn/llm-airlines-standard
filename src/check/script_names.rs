use std::collections::BTreeMap;
use std::sync::LazyLock;

use regex::Regex;

use super::names::{Name, NameKind};

static FUNCTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bfunction\s*\*?\s*([A-Za-z_$][\w$]*)\s*[<(]").expect("the pattern is a valid regex")
});
static ARROW: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*(?::[^=\n]+)?=\s*(?:async\s+)?(?:function\b|\([^)]*\)\s*(?::[^=\n]+)?=>|[A-Za-z_$][\w$]*\s*=>)",
    )
    .expect("the pattern is a valid regex")
});
static VARIABLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:const|let|var)\s+([A-Za-z_$][\w$]*)").expect("the pattern is a valid regex"));
static TYPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:class|interface|type|enum)\s+([A-Za-z_$][\w$]*)").expect("the pattern is a valid regex")
});
static METHOD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?m)^[ \t]*(?:(?:public|private|protected|static|async|override|readonly|abstract)\s+)*(get\s+|set\s+)?(#?[A-Za-z_$][\w$]*)\s*(?:<[^>\n]*>)?\s*\([^;\n]*\)\s*(?::\s*[^={;\n]+)?\{\s*$",
    )
    .expect("the pattern is a valid regex")
});
const KEYWORDS: [&str; 10] =
    ["if", "for", "while", "switch", "catch", "function", "return", "constructor", "with", "else"];

fn is_pascal_case(name: &str) -> bool {
    name.starts_with(|character: char| character.is_ascii_uppercase())
        && name.chars().any(|character| character.is_ascii_lowercase())
}

pub fn find(code_text: &str) -> Vec<Name> {
    let mut functions: BTreeMap<usize, Name> = BTreeMap::new();
    for pattern in [&*FUNCTION, &*ARROW] {
        for found in pattern.captures_iter(code_text).filter_map(|captures| captures.get(1)) {
            functions.insert(found.start(), Name::new(&found, NameKind::Function, is_pascal_case(found.as_str())));
        }
    }
    let mut names: Vec<Name> = functions.values().cloned().collect();
    for captures in METHOD.captures_iter(code_text) {
        let is_accessor = captures.get(1).is_some();
        let methods = captures.get(2).filter(|found| !KEYWORDS.contains(&found.as_str()));
        names.extend(methods.map(|found| Name::new(&found, NameKind::Function, is_accessor)));
    }
    let variables = VARIABLE.captures_iter(code_text).filter_map(|captures| captures.get(1));
    names.extend(
        variables
            .filter(|found| !functions.contains_key(&found.start()))
            .map(|found| Name::new(&found, NameKind::Variable, false)),
    );
    names.extend(
        TYPE.captures_iter(code_text)
            .filter_map(|captures| captures.get(1))
            .map(|found| Name::new(&found, NameKind::Type, false)),
    );
    names
}
