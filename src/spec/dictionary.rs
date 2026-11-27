use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WordEntry {
    pub word: String,
    #[serde(default)]
    pub meaning: String,
    #[serde(default)]
    pub not_for: String,
    #[serde(default)]
    pub example: String,
    #[serde(default, rename = "use")]
    pub replacement: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Abbreviations {
    pub approved: Vec<String>,
    pub avoid: Vec<WordEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AvoidPhrase {
    pub text: String,
    #[serde(rename = "use")]
    pub replacement: String,
    pub rule: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProseWords {
    pub avoid: Vec<AvoidPhrase>,
    pub hedges: Vec<String>,
    pub imperatives: Vec<String>,
    pub participles: Vec<String>,
    pub stative: Vec<String>,
    pub ing_exceptions: Vec<String>,
    pub period_abbreviations: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub title: String,
    pub extensions: Vec<String>,
    pub verbs: Vec<String>,
    pub exempt_names: Vec<String>,
    pub exempt_prefixes: Vec<String>,
    pub handler_prefix_extensions: Vec<String>,
    pub allowed_abbreviations: Vec<String>,
    pub noun_kinds: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dictionary {
    pub verbs: Vec<WordEntry>,
    pub predicates: Vec<WordEntry>,
    pub prefixes: Vec<WordEntry>,
    pub avoid_verbs: Vec<WordEntry>,
    pub avoid_nouns: Vec<WordEntry>,
    pub abbreviations: Abbreviations,
    pub synonym_groups: Vec<Vec<String>>,
    pub vague_messages: Vec<String>,
    pub directives: Vec<String>,
    pub prose: ProseWords,
    pub profiles: BTreeMap<String, Profile>,
}

impl Dictionary {
    pub fn find_language(&self, extension: &str) -> Option<(&str, &Profile)> {
        self.profiles
            .iter()
            .find(|(_, profile)| profile.extensions.iter().any(|known| known == extension))
            .map(|(name, profile)| (name.as_str(), profile))
    }
}
