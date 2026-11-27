use std::collections::BTreeMap;

pub fn parse(text: &str) -> BTreeMap<String, String> {
    let mut lines = text.lines();
    if lines.next().map(str::trim) != Some("---") {
        return BTreeMap::new();
    }
    let mut fields = BTreeMap::new();
    for line in lines {
        if line.trim() == "---" {
            return fields;
        }
        if let Some((key, value)) = line.split_once(':') {
            let value = value.trim().trim_matches(|character| character == '\'' || character == '"');
            fields.insert(key.trim().to_string(), value.to_string());
        }
    }
    BTreeMap::new()
}
