use super::finding::Finding;
use super::names::{Name, NameKind};
use super::vocabulary::Vocabulary;
use super::words;
use crate::spec::dictionary::Profile;
use crate::spec::Standard;

const NEGATIVE_WORDS: [&str; 3] = ["not", "no", "non"];
const EVENT_PREFIX: &str = "on";
const TRY_PREFIX: &str = "try";
const CALLBACK_PREFIX: &str = "handle";
const MIN_NAME_CHARS: usize = 2;

pub struct CodeContext<'a> {
    pub label: String,
    pub extension: String,
    pub profile: &'a Profile,
    pub standard: &'a Standard,
    pub vocabulary: &'a Vocabulary,
}

pub fn is_exempt(name: &Name, context: &CodeContext) -> bool {
    let is_dunder = name.name.starts_with("__") && name.name.ends_with("__");
    let profile = context.profile;
    is_dunder
        || profile.exempt_names.contains(&name.name)
        || profile.exempt_prefixes.iter().any(|prefix| name.name.starts_with(prefix.as_str()))
}

fn find_vague_words(name: &Name, words: &[String], context: &CodeContext) -> Vec<(&'static str, String)> {
    let vocabulary = context.vocabulary;
    let mut found = Vec::new();
    if let Some(replacement) =
        words.first().filter(|_| name.kind == NameKind::Function).and_then(|word| vocabulary.avoid_verbs.get(word))
    {
        found.push(("C1.6", format!("\"{}\" is a vague verb. Use {replacement}.", words[0])));
    }
    for word in words {
        if let Some(replacement) = vocabulary.avoid_nouns.get(word) {
            found.push(("C1.6", format!("\"{word}\" is a vague word. Use {replacement}.")));
        }
    }
    found
}

fn find_abbreviations(words: &[String], context: &CodeContext) -> Vec<(&'static str, String)> {
    let vocabulary = context.vocabulary;
    let is_allowed = |word: &String| {
        vocabulary.approved_abbreviations.contains(word) || context.profile.allowed_abbreviations.contains(word)
    };
    words
        .iter()
        .filter(|word| !is_allowed(word))
        .filter_map(|word| {
            vocabulary
                .avoid_abbreviations
                .get(word)
                .map(|replacement| ("C1.7", format!("\"{word}\" is an abbreviation. Write {replacement}.")))
        })
        .collect()
}

fn find_long_name(name: &Name, words: &[String], context: &CodeContext) -> Vec<(&'static str, String)> {
    let limit = context.standard.get_limit("code", "name_words");
    let is_test = context.profile.exempt_prefixes.iter().any(|prefix| name.name.starts_with(prefix.as_str()));
    if words.len() <= limit || is_test {
        return Vec::new();
    }
    vec![(
        "C2.1",
        format!("This name has {} words. The limit is {limit}. Move context into a module or a type.", words.len()),
    )]
}

fn find_verb_start(words: &[String]) -> &str {
    match words {
        [first, second, ..] if first == TRY_PREFIX => second,
        [first, ..] => first,
        [] => "",
    }
}

fn is_approved_start(words: &[String], context: &CodeContext) -> bool {
    let vocabulary = context.vocabulary;
    let first = find_verb_start(words);
    let is_callback =
        first == CALLBACK_PREFIX && context.profile.handler_prefix_extensions.contains(&context.extension);
    let is_known = vocabulary.verbs.contains(first)
        || vocabulary.predicates.contains(first)
        || context.profile.verbs.iter().any(|verb| verb == first);
    first == EVENT_PREFIX || is_callback || is_known || vocabulary.avoid_verbs.contains_key(first)
}

fn find_missing_verb(name: &Name, words: &[String], context: &CodeContext) -> Vec<(&'static str, String)> {
    if name.kind != NameKind::Function || name.is_noun_allowed || is_approved_start(words, context) {
        return Vec::new();
    }
    let first = find_verb_start(words);
    let message = format!(
        "\"{}\" does not start with an approved verb. Use a verb from the dictionary, or record \"{first}\" as a technical verb in LEXICON.md.",
        name.name
    );
    vec![("C3.1", message)]
}

fn find_negative_name(words: &[String], context: &CodeContext) -> Vec<(&'static str, String)> {
    let is_predicate_negative = words.len() > 1
        && context.vocabulary.predicates.contains(&words[0])
        && NEGATIVE_WORDS.contains(&words[1].as_str());
    let is_negative =
        words.iter().any(|word| word == "not") || (words.len() > 1 && words[0] == "no") || is_predicate_negative;
    if !is_negative {
        return Vec::new();
    }
    vec![("C4.4", String::from("This name is negative. Use the positive form, such as isEnabled for isNotDisabled."))]
}

pub fn find_name_findings(name: &Name, line: usize, context: &CodeContext) -> Vec<Finding> {
    let words: Vec<String> = words::split(&name.name).iter().map(|word| word.to_lowercase()).collect();
    if name.name.len() < MIN_NAME_CHARS || words.is_empty() || is_exempt(name, context) {
        return Vec::new();
    }
    let mut results = find_vague_words(name, &words, context);
    results.extend(find_abbreviations(&words, context));
    results.extend(find_long_name(name, &words, context));
    results.extend(find_missing_verb(name, &words, context));
    results.extend(find_negative_name(&words, context));
    results.into_iter().map(|(rule, message)| Finding::new(line, rule, message)).collect()
}
