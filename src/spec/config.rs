use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::rules::Limits;
use crate::disk::read_text;
use crate::error::{Error, Result};

pub const CONFIG_NAME: &str = ".ste.json";

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub lexicon: Option<String>,
    #[serde(default)]
    pub disable: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub limits: Limits,
}

pub fn find_path(start: &Path) -> Option<PathBuf> {
    start.ancestors().map(|folder| folder.join(CONFIG_NAME)).find(|candidate| candidate.is_file())
}

pub fn load(path: &Path) -> Result<Config> {
    let text = read_text(path)?;
    serde_json::from_str(&text).map_err(|error| Error::new(format!("Cannot parse {}: {error}.", path.display())))
}
