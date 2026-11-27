use std::path::{Path, PathBuf};

use super::code_rules::{
    find_comment_findings, find_error_messages, find_file_length, find_function_lengths, find_suppressions,
    is_view_ignored, SourceView,
};
use super::files::{self, to_display_path, FileQuery};
use super::finding::{filter_findings, Finding};
use super::lexer::{list_tokens, Language};
use super::name_rules::{find_name_findings, is_exempt, CodeContext};
use super::names::{self, Name, NameKind};
use super::rotation::{find_rotations, render_lexicon, NameUse};
use super::source::{build_code_text, build_line_index, find_docstrings};
use super::vocabulary::{self, Vocabulary};
use super::words;
use crate::disk::read_text;
use crate::error::Result;
use crate::spec::Standard;

fn read_view(path: &Path, language: Language) -> Result<SourceView> {
    let text = read_text(path)?;
    let tokens = list_tokens(&text, language);
    let code_text = build_code_text(&text, &tokens);
    let docstrings =
        if language == Language::Python { find_docstrings(&text, &tokens, &code_text) } else { Vec::new() };
    let line_index = build_line_index(&text);
    Ok(SourceView { language, text, code_text, tokens, docstrings, line_index })
}

fn find_extension(path: &Path) -> String {
    path.extension().map(|extension| format!(".{}", extension.to_string_lossy().to_lowercase())).unwrap_or_default()
}

fn list_name_uses(view: &SourceView, names: &[Name], context: &CodeContext) -> Vec<NameUse> {
    names
        .iter()
        .filter(|name| name.kind == NameKind::Function && !name.is_noun_allowed && !is_exempt(name, context))
        .map(|name| NameUse {
            name: name.name.clone(),
            words: words::split(&name.name).iter().map(|word| word.to_lowercase()).collect(),
            path: context.label.clone(),
            line: view.get_line_number(name.offset),
            language: format!("{:?}", view.language),
        })
        .collect()
}

struct FileResult {
    findings: Vec<Finding>,
    uses: Vec<NameUse>,
}

fn validate_file(path: &Path, standard: &Standard, vocabulary: &Vocabulary) -> Result<FileResult> {
    let extension = find_extension(path);
    let (language_name, profile) = standard
        .dictionary
        .find_language(&extension)
        .unwrap_or(("typescript", &standard.dictionary.profiles["typescript"]));
    let view = read_view(path, Language::from_profile(language_name))?;
    if is_view_ignored(&view) {
        return Ok(FileResult { findings: Vec::new(), uses: Vec::new() });
    }
    let label = to_display_path(path);
    let context = CodeContext { label: label.clone(), extension, profile, standard, vocabulary };
    let names = names::find(&view.code_text, view.language);
    let mut findings = find_comment_findings(&view, &context);
    findings.extend(find_file_length(&view, &context));
    findings.extend(find_function_lengths(&view, &names, &context));
    findings.extend(find_error_messages(&view, &context));
    findings
        .extend(names.iter().flat_map(|name| find_name_findings(name, view.get_line_number(name.offset), &context)));
    let located = findings.into_iter().map(|finding| finding.with_path(&label)).collect();
    let uses = list_name_uses(&view, &names, &context);
    Ok(FileResult { findings: filter_findings(located, standard, &find_suppressions(&view)), uses })
}

fn list_code_files(paths: &[PathBuf], standard: &Standard) -> Vec<PathBuf> {
    let extensions: Vec<String> =
        standard.dictionary.profiles.values().flat_map(|profile| profile.extensions.iter().cloned()).collect();
    let query = FileQuery { extensions: &extensions, root: &standard.root, excluded: &standard.excluded };
    files::list(paths, &query)
}

fn validate_files(paths: &[PathBuf], standard: &Standard, vocabulary: &Vocabulary) -> Result<FileResult> {
    let mut total = FileResult { findings: Vec::new(), uses: Vec::new() };
    for path in list_code_files(paths, standard) {
        let result = validate_file(&path, standard, vocabulary)?;
        total.findings.extend(result.findings);
        total.uses.extend(result.uses);
    }
    Ok(total)
}

pub fn run(paths: &[PathBuf], standard: &Standard) -> Result<Vec<Finding>> {
    let vocabulary = vocabulary::build(standard);
    let mut total = validate_files(paths, standard, &vocabulary)?;
    let rotations = find_rotations(&total.uses, &vocabulary, &standard.dictionary.synonym_groups);
    total.findings.extend(filter_findings(rotations, standard, &Default::default()));
    Ok(total.findings)
}

pub fn run_lexicon(paths: &[PathBuf], standard: &Standard) -> Result<String> {
    let vocabulary = vocabulary::build(standard);
    let total = validate_files(paths, standard, &vocabulary)?;
    Ok(render_lexicon(&total.uses, &vocabulary))
}
