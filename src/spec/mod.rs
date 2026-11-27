pub mod config;
pub mod dictionary;
pub mod frontmatter;
pub mod lexicon;
pub mod rules;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::disk::read_text;
use crate::embedded::get_embedded_text;
use crate::error::Result;
use config::Config;
use dictionary::Dictionary;
use lexicon::Lexicon;
use rules::{build_limits, Limits, Rule};

pub const WRITING_RULES: &str = "spec/writing/rules.md";
pub const CODE_RULES: &str = "spec/code/rules.md";
pub const DICTIONARY: &str = "spec/dictionary.json";

#[derive(Debug, Clone)]
pub struct Standard {
    pub severities: BTreeMap<String, String>,
    pub limits: Limits,
    pub dictionary: Dictionary,
    pub root: PathBuf,
    pub disabled: BTreeSet<String>,
    pub excluded: Vec<String>,
    pub lexicon: Lexicon,
}

impl Standard {
    pub fn get_limit(&self, group: &str, key: &str) -> usize {
        let value = self.limits.get(group).and_then(|values| values.get(key)).copied().unwrap_or(u64::MAX);
        usize::try_from(value).unwrap_or(usize::MAX)
    }
}

pub fn load_embedded_rules() -> Result<Vec<Rule>> {
    let mut rules = rules::parse(get_embedded_text(WRITING_RULES), WRITING_RULES)?;
    rules.extend(rules::parse(get_embedded_text(CODE_RULES), CODE_RULES)?);
    Ok(rules)
}

pub fn load_embedded_dictionary() -> Result<Dictionary> {
    Ok(serde_json::from_str(get_embedded_text(DICTIONARY))?)
}

fn merge_limits(base: &mut Limits, overrides: &Limits) {
    for (group, values) in overrides {
        base.entry(group.clone()).or_default().extend(values.iter().map(|(key, value)| (key.clone(), *value)));
    }
}

fn load_lexicon(path: &Path) -> Result<Lexicon> {
    if !path.is_file() {
        return Ok(Lexicon::default());
    }
    Ok(lexicon::parse(&read_text(path)?))
}

pub fn load_standard(config_path: Option<&Path>) -> Result<Standard> {
    let rules = load_embedded_rules()?;
    let mut limits = build_limits(&rules);
    let current = std::env::current_dir().unwrap_or_default();
    let found = config_path.map(Path::to_path_buf).or_else(|| config::find_path(&current));
    let config: Config = found.as_deref().map(config::load).transpose()?.unwrap_or_default();
    let parent = found
        .as_deref()
        .and_then(Path::parent)
        .filter(|folder| !folder.as_os_str().is_empty())
        .map_or(current, Path::to_path_buf);
    let root = std::fs::canonicalize(&parent).unwrap_or(parent);
    merge_limits(&mut limits, &config.limits);
    let lexicon = load_lexicon(&root.join(config.lexicon.as_deref().unwrap_or("LEXICON.md")))?;
    Ok(Standard {
        severities: rules.into_iter().map(|rule| (rule.id, rule.severity)).collect(),
        limits,
        dictionary: load_embedded_dictionary()?,
        root,
        disabled: config.disable.into_iter().collect(),
        excluded: config.exclude,
        lexicon,
    })
}
