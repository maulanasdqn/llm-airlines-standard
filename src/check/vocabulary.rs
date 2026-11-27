use std::collections::{BTreeMap, BTreeSet};

use crate::spec::dictionary::WordEntry;
use crate::spec::Standard;

#[derive(Debug, Clone)]
pub struct Vocabulary {
    pub verbs: BTreeSet<String>,
    pub predicates: BTreeSet<String>,
    pub avoid_verbs: BTreeMap<String, String>,
    pub avoid_nouns: BTreeMap<String, String>,
    pub avoid_abbreviations: BTreeMap<String, String>,
    pub approved_abbreviations: BTreeSet<String>,
}

fn list_words(entries: &[WordEntry]) -> BTreeSet<String> {
    entries.iter().map(|entry| entry.word.to_lowercase()).collect()
}

fn map_words(entries: &[WordEntry], allowed: &BTreeSet<String>) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|entry| (entry.word.to_lowercase(), entry.replacement.clone()))
        .filter(|(word, _)| !allowed.contains(word))
        .collect()
}

pub fn build(standard: &Standard) -> Vocabulary {
    let words = &standard.dictionary;
    let lexicon = &standard.lexicon;
    let allowed: BTreeSet<String> =
        lexicon.nouns.keys().chain(lexicon.verbs.keys()).chain(lexicon.abbreviations.keys()).cloned().collect();
    let mut avoid_nouns = map_words(&words.avoid_nouns, &allowed);
    avoid_nouns.extend(lexicon.avoid.clone());
    let mut approved_abbreviations: BTreeSet<String> = words.abbreviations.approved.iter().cloned().collect();
    approved_abbreviations.extend(lexicon.abbreviations.keys().cloned());
    let mut verbs = list_words(&words.verbs);
    verbs.extend(lexicon.verbs.keys().cloned());
    Vocabulary {
        verbs,
        predicates: list_words(&words.predicates),
        avoid_verbs: map_words(&words.avoid_verbs, &allowed),
        avoid_nouns,
        avoid_abbreviations: map_words(&words.abbreviations.avoid, &allowed),
        approved_abbreviations,
    }
}
