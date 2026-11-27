use std::sync::LazyLock;

use regex::Regex;
use serde_json::{json, Value};

use crate::error::{Error, Result};
use crate::spec::dictionary::Dictionary;
use crate::spec::rules::Limits;

static PLACEHOLDER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{\{\s*([\w.]+)\s*\}\}").expect("the pattern is a valid regex"));
const BE_FORMS: [&str; 8] = ["am", "is", "are", "was", "were", "be", "been", "being"];
const PROGRESSIVE_FORMS: usize = 7;

fn find_value<'a>(context: &'a Value, dotted: &str) -> Result<&'a Value> {
    dotted.split('.').try_fold(context, |value, part| {
        value.get(part).ok_or_else(|| {
            Error::new(format!("The template uses \"{{{{ {dotted} }}}}\", but the build has no value for it."))
        })
    })
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Array(items) => format!("[{}]", items.iter().map(Value::to_string).collect::<Vec<String>>().join(", ")),
        other => other.to_string(),
    }
}

pub fn render_template(text: &str, context: &Value) -> Result<String> {
    let mut rendered = String::with_capacity(text.len());
    let mut last = 0;
    for found in PLACEHOLDER.captures_iter(text) {
        let whole = found.get(0).map_or(0..0, |matched| matched.range());
        rendered.push_str(&text[last..whole.start]);
        rendered.push_str(&format_value(find_value(context, &found[1])?));
        last = whole.end;
    }
    rendered.push_str(&text[last..]);
    Ok(rendered)
}

fn escape_yaml(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

fn build_exceptions(words: &[String], forms: &[&str]) -> Vec<String> {
    forms.iter().flat_map(|form| words.iter().map(move |word| format!("{form} {word}"))).collect()
}

fn build_vale_lists(dictionary: &Dictionary) -> Value {
    let prose = &dictionary.prose;
    let swap: Vec<String> = prose
        .avoid
        .iter()
        .map(|entry| format!("  {}: {}", escape_yaml(&regex::escape(&entry.text)), escape_yaml(&entry.replacement)))
        .collect();
    let participles = prose.participles.join("|");
    let progressive_forms = &BE_FORMS[..PROGRESSIVE_FORMS];
    json!({
        "swap": swap.join("\n"),
        "hedges": prose.hedges.join("|"),
        "passive": format!(r"(?:{})\s+(?:\w+ly\s+)?(?:\w+ed|{participles})", BE_FORMS.join("|")),
        "perfect": format!(r"(?:has|have|had)\s+(?:\w+ly\s+)?(?:\w+ed|{participles})"),
        "progressive": format!(r"(?:{})\s+\w+ing", progressive_forms.join("|")),
        "passive_exceptions": build_exceptions(&prose.stative, &BE_FORMS),
        "progressive_exceptions": build_exceptions(&prose.ing_exceptions, progressive_forms),
    })
}

fn build_denied_names(dictionary: &Dictionary) -> Value {
    let words: Vec<&String> =
        dictionary.avoid_nouns.iter().chain(dictionary.abbreviations.avoid.iter()).map(|entry| &entry.word).collect();
    let lists: serde_json::Map<String, Value> = dictionary
        .profiles
        .iter()
        .map(|(language, profile)| {
            let mut denied: Vec<&String> =
                words.iter().copied().filter(|word| !profile.allowed_abbreviations.contains(word)).collect();
            denied.sort();
            denied.dedup();
            (language.clone(), json!(denied))
        })
        .collect();
    Value::Object(lists)
}

pub fn build_context(limits: &Limits, dictionary: &Dictionary) -> Value {
    let nesting = limits.get("code").and_then(|values| values.get("nesting_depth")).copied().unwrap_or_default();
    json!({
        "limits": limits,
        "derived": { "clippy_nesting": nesting + 1 },
        "lists": { "denied_names": build_denied_names(dictionary), "vale": build_vale_lists(dictionary) },
    })
}
