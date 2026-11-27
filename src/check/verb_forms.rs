use std::collections::BTreeSet;
use std::sync::LazyLock;

use regex::Regex;

use crate::spec::dictionary::ProseWords;

static PASSIVE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:am|is|are|was|were|be|been|being)\s+(?:not\s+)?(?:\w+ly\s+)?([a-z]+)\b")
        .expect("the pattern is a valid regex")
});
static PERFECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:has|have|had)\s+(?:not\s+)?(?:\w+ly\s+)?([a-z]+)\b").expect("the pattern is a valid regex")
});
static PROGRESSIVE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:am|is|are|was|were|be|been)\s+(?:not\s+)?([a-z]+ing)\b").expect("the pattern is a valid regex")
});

const SHORT_WORD_CHARS: usize = 3;

pub struct VerbWords {
    pub participles: BTreeSet<String>,
    pub stative: BTreeSet<String>,
    pub ing_exceptions: BTreeSet<String>,
}

impl VerbWords {
    pub fn from_prose(prose: &ProseWords) -> Self {
        let collect = |words: &[String]| words.iter().cloned().collect::<BTreeSet<String>>();
        Self {
            participles: collect(&prose.participles),
            stative: collect(&prose.stative),
            ing_exceptions: collect(&prose.ing_exceptions),
        }
    }

    fn is_participle(&self, word: &str) -> bool {
        (word.ends_with("ed") && word.len() > SHORT_WORD_CHARS) || self.participles.contains(word)
    }
}

pub fn find_passive(text: &str, words: &VerbWords) -> Option<String> {
    PASSIVE.captures_iter(text).find_map(|found| {
        let word = &found[1];
        (words.is_participle(word) && !words.stative.contains(word))
            .then(|| format!("Passive voice: \"{}\". Name the actor and use the active voice.", &found[0]))
    })
}

pub fn find_perfect(text: &str, words: &VerbWords) -> Option<String> {
    PERFECT.captures_iter(text).find_map(|found| {
        words
            .is_participle(&found[1])
            .then(|| format!("Compound verb form: \"{}\". Use the simple past or the simple present.", &found[0]))
    })
}

pub fn find_progressive(text: &str, words: &VerbWords) -> Option<String> {
    PROGRESSIVE.captures_iter(text).find_map(|found| {
        (!words.ing_exceptions.contains(&found[1]))
            .then(|| format!("\"-ing\" verb form: \"{}\". Use the simple present or the simple past.", &found[0]))
    })
}
