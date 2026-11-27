use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct Lexicon {
    pub verbs: BTreeMap<String, String>,
    pub nouns: BTreeMap<String, String>,
    pub abbreviations: BTreeMap<String, String>,
    pub avoid: BTreeMap<String, String>,
}

fn find_section<'a>(lexicon: &'a mut Lexicon, heading: &str) -> Option<&'a mut BTreeMap<String, String>> {
    match heading.trim().to_lowercase().as_str() {
        "technical verbs" => Some(&mut lexicon.verbs),
        "technical nouns" => Some(&mut lexicon.nouns),
        "approved abbreviations" => Some(&mut lexicon.abbreviations),
        "words to avoid" => Some(&mut lexicon.avoid),
        _ => None,
    }
}

fn split_row(line: &str) -> Vec<String> {
    let inner = line.trim().trim_matches('|');
    inner.split('|').map(|cell| cell.trim().trim_matches('`').trim().to_string()).collect()
}

fn is_table_body(line: &str) -> bool {
    let stripped = line.trim();
    let is_separator = stripped.chars().all(|character| "|-: ".contains(character));
    stripped.starts_with('|') && !is_separator
}

pub fn parse(text: &str) -> Lexicon {
    let mut lexicon = Lexicon::default();
    let mut section: Option<String> = None;
    let mut rows_seen = 0;
    for line in text.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            section = Some(heading.to_string());
            rows_seen = 0;
            continue;
        }
        let Some(heading) = section.as_deref().filter(|_| is_table_body(line)) else {
            continue;
        };
        rows_seen += 1;
        let cells = split_row(line);
        let Some(target) = find_section(&mut lexicon, heading).filter(|_| rows_seen > 1 && !cells[0].is_empty()) else {
            continue;
        };
        target.insert(cells[0].to_lowercase(), cells.get(1).cloned().unwrap_or_default());
    }
    lexicon
}
