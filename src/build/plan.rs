use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Value};

use super::templating::{build_context, render_template};
use super::views::{render_dictionary, render_json, render_mapping, render_words};
use crate::disk::{list_tree, read_bytes, read_text};
use crate::error::Result;
use crate::spec::dictionary::Dictionary;
use crate::spec::rules::{self, build_limits, Rule};

pub type Outputs = BTreeMap<String, Vec<u8>>;

pub const PROFILES: [&str; 4] = ["typescript", "python", "go", "rust"];
pub const OWNED_FOLDERS: [&str; 3] = ["templates/agents", "skills/ste-adopt/assets", "linters"];
pub const EXECUTABLE_NAMES: [&str; 1] = ["commit-msg"];
pub const INSTRUCTION_NAMES: [&str; 2] = ["AGENTS.md", "CLAUDE.md"];
const MIRRORS: [(&str, &str); 2] =
    [("templates/agents", "skills/ste-adopt/assets/agents"), ("linters", "skills/ste-adopt/assets/linters")];
const LINTS_HEADER: &str = "[lints.rust]";

fn list_copies() -> Vec<(String, String)> {
    let mut copies: Vec<(&str, &str)> = vec![
        ("spec/writing/rules.md", "skills/ste-writing/references/rules.md"),
        ("spec/writing/rules.md", "skills/ste-code/references/writing-rules.md"),
        ("spec/writing/rules.md", "skills/ste-review/references/writing-rules.md"),
        ("spec/writing/examples.md", "skills/ste-writing/references/examples.md"),
        ("spec/writing/words.md", "skills/ste-writing/references/words.md"),
        ("spec/writing/words.md", "skills/ste-review/references/words.md"),
        ("spec/code/rules.md", "skills/ste-code/references/rules.md"),
        ("spec/code/rules.md", "skills/ste-review/references/code-rules.md"),
        ("spec/code/examples.md", "skills/ste-code/references/examples.md"),
        ("spec/code/dictionary.md", "skills/ste-code/references/dictionary.md"),
        ("spec/code/dictionary.md", "skills/ste-review/references/dictionary.md"),
        ("spec/code/mapping.md", "skills/ste-code/references/mapping.md"),
        ("spec/code/mapping.md", "skills/ste-review/references/mapping.md"),
        ("templates/LEXICON.md", "skills/ste-adopt/assets/LEXICON.md"),
        ("linters/rust/clippy.toml", "clippy.toml"),
    ];
    let profiles: Vec<(String, String)> = PROFILES
        .iter()
        .map(|profile| {
            (format!("spec/code/profiles/{profile}.md"), format!("skills/ste-code/references/profiles/{profile}.md"))
        })
        .collect();
    copies.drain(..).map(|(source, target)| (source.to_string(), target.to_string())).chain(profiles).collect()
}

fn list_recipe(recipe: &str) -> Vec<String> {
    let parts: Vec<String> = match recipe {
        "base" => vec!["prose".into(), "code".into()],
        "docs" => vec!["prose".into(), "docs".into()],
        profile => vec!["prose".into(), "code".into(), format!("profiles/{profile}")],
    };
    std::iter::once("header".to_string()).chain(parts).chain(std::iter::once("footer".to_string())).collect()
}

fn build_spec_outputs(root: &Path, outputs: &mut Outputs) -> Result<Value> {
    let mut rules: Vec<Rule> = rules::parse(&read_text(&root.join("spec/writing/rules.md"))?, "spec/writing/rules.md")?;
    rules.extend(rules::parse(&read_text(&root.join("spec/code/rules.md"))?, "spec/code/rules.md")?);
    let limits = build_limits(&rules);
    let dictionary: Dictionary = serde_json::from_str(&read_text(&root.join("spec/dictionary.json"))?)?;
    outputs.insert("spec/limits.json".into(), render_json(&limits).into_bytes());
    outputs.insert("spec/code/dictionary.md".into(), render_dictionary(&dictionary).into_bytes());
    outputs.insert("spec/writing/words.md".into(), render_words(&dictionary).into_bytes());
    outputs.insert("spec/code/mapping.md".into(), render_mapping(&rules).into_bytes());
    let mut context = build_context(&limits, &dictionary);
    let titles: serde_json::Map<String, Value> =
        dictionary.profiles.iter().map(|(key, profile)| (key.clone(), json!(profile.title))).collect();
    context["titles"] = Value::Object(titles);
    Ok(context)
}

fn build_agent_outputs(root: &Path, context: &Value, outputs: &mut Outputs) -> Result<()> {
    for recipe in ["base", "docs"].into_iter().chain(PROFILES) {
        let title = match recipe {
            "base" => "any language".to_string(),
            "docs" => "documentation".to_string(),
            profile => context["titles"][profile].as_str().unwrap_or_default().to_string(),
        };
        let mut recipe_context = context.clone();
        recipe_context["profile"] = json!({ "key": recipe, "title": title });
        let parts = list_recipe(recipe)
            .iter()
            .map(|part| read_text(&root.join(format!("templates/src/{part}.md"))))
            .collect::<Result<Vec<String>>>()?;
        let text =
            parts.iter().map(|part| format!("{}\n", part.trim_end_matches('\n'))).collect::<Vec<String>>().join("\n");
        outputs.insert(
            format!("templates/agents/{recipe}/AGENTS.md"),
            render_template(&text, &recipe_context)?.into_bytes(),
        );
    }
    Ok(())
}

fn build_linter_outputs(root: &Path, context: &Value, outputs: &mut Outputs) -> Result<()> {
    let source = root.join("linters/src");
    for relative in list_tree(&source) {
        let text = read_text(&source.join(&relative))?;
        let target = relative.strip_suffix(".tmpl").unwrap_or(&relative);
        outputs.insert(format!("linters/{target}"), render_template(&text, context)?.into_bytes());
    }
    Ok(())
}

fn find_output_text(outputs: &Outputs, path: &str) -> String {
    outputs.get(path).map(|bytes| String::from_utf8_lossy(bytes).into_owned()).unwrap_or_default()
}

fn build_cargo_outputs(root: &Path, outputs: &mut Outputs) -> Result<()> {
    let lints = find_output_text(outputs, "linters/rust/lints.toml");
    let template = read_text(&root.join("tests/fixtures/code/rust/Cargo.toml.tmpl"))?;
    for kind in ["good", "bad"] {
        let context = json!({ "crate": format!("ste-fixture-{kind}"), "lints": lints.trim() });
        outputs.insert(
            format!("tests/fixtures/code/rust/{kind}/Cargo.toml"),
            render_template(&template, &context)?.into_bytes(),
        );
    }
    let manifest = read_text(&root.join("Cargo.toml"))?;
    let head = manifest.split(LINTS_HEADER).next().unwrap_or_default().trim_end();
    outputs.insert("Cargo.toml".into(), format!("{head}\n\n{lints}").into_bytes());
    Ok(())
}

fn find_source(root: &Path, outputs: &Outputs, relative: &str) -> Result<Vec<u8>> {
    outputs.get(relative).map_or_else(|| read_bytes(&root.join(relative)), |bytes| Ok(bytes.clone()))
}

fn build_copy_outputs(root: &Path, outputs: &mut Outputs) -> Result<()> {
    let mut copies = Outputs::new();
    for (source, target) in list_copies() {
        copies.insert(target, find_source(root, outputs, &source)?);
    }
    for (folder, target_folder) in MIRRORS {
        let instructions: Vec<String> = INSTRUCTION_NAMES.iter().map(|name| format!("{folder}/{name}")).collect();
        let sources = outputs.keys().filter(|path| {
            path.starts_with(&format!("{folder}/")) && !path.starts_with("linters/src/") && !instructions.contains(path)
        });
        for source in sources {
            copies.insert(format!("{target_folder}{}", &source[folder.len()..]), outputs[source].clone());
        }
    }
    outputs.extend(copies);
    Ok(())
}

pub fn build_outputs(root: &Path) -> Result<Outputs> {
    let mut outputs = Outputs::new();
    let context = build_spec_outputs(root, &mut outputs)?;
    build_agent_outputs(root, &context, &mut outputs)?;
    build_linter_outputs(root, &context, &mut outputs)?;
    build_cargo_outputs(root, &mut outputs)?;
    build_copy_outputs(root, &mut outputs)?;
    Ok(outputs)
}
