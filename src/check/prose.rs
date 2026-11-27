use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::files::{self, to_display_path, FileQuery};
use super::finding::{filter_findings, Finding, Suppressions};
use super::inline;
use super::markdown;
use super::prose_rules::{find_block_findings, find_sentence_findings, ProseContext};
use super::sentences::{self, SentenceWords};
use crate::disk::read_text;
use crate::error::Result;
use crate::spec::Standard;

const PROSE_EXTENSIONS: [&str; 4] = [".md", ".markdown", ".mdx", ".txt"];

pub fn find_text_findings(text: &str, context: &ProseContext) -> (Vec<Finding>, Suppressions) {
    let state = markdown::parse(text);
    if state.is_ignored {
        return (Vec::new(), Suppressions::new());
    }
    let abbreviations: BTreeSet<String> = context.prose.period_abbreviations.iter().cloned().collect();
    let words = SentenceWords { abbreviations: &abbreviations, imperatives: &context.imperatives };
    let mut findings = Vec::new();
    for block in &state.blocks {
        let cleaned: Vec<String> = block.lines.iter().map(|(_, line)| inline::strip(line)).collect();
        let numbers: Vec<usize> = block.lines.iter().map(|(number, _)| *number).collect();
        let sentences = sentences::split(&cleaned.join("\n"), &numbers, &words);
        findings.extend(find_block_findings(block, &sentences, context));
        findings.extend(sentences.iter().flat_map(|sentence| find_sentence_findings(sentence, context)));
    }
    (findings, state.suppressions)
}

fn find_file_findings(path: &Path, context: &ProseContext) -> Result<Vec<Finding>> {
    let label = to_display_path(path);
    let (findings, suppressions) = find_text_findings(&read_text(path)?, context);
    let located = findings.into_iter().map(|finding| finding.with_path(&label)).collect();
    Ok(filter_findings(located, context.standard, &suppressions))
}

pub fn run(paths: &[PathBuf], standard: &Standard) -> Result<Vec<Finding>> {
    let context = ProseContext::new(standard);
    let extensions: Vec<String> = PROSE_EXTENSIONS.iter().map(ToString::to_string).collect();
    let query = FileQuery { extensions: &extensions, root: &standard.root, excluded: &standard.excluded };
    let mut findings = Vec::new();
    for path in files::list(paths, &query) {
        findings.extend(find_file_findings(&path, &context)?);
    }
    Ok(findings)
}
