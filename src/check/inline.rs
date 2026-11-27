use std::sync::LazyLock;

use regex::Regex;

static INLINE_RULES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    [
        (r"`[^`]*`", "CODE"),
        ("\"[^\"]*\"|\u{201c}[^\u{201d}]*\u{201d}", "QUOTE"),
        (r"!\[[^\]]*\]\([^)]*\)", ""),
        (r"\[([^\]]*)\]\([^)]*\)", "$1"),
        (r"<https?://[^>]+>|https?://\S+", "LINK"),
        (r"<[^>]+>", ""),
        (r"\*\*|__|\*", ""),
        (r"[ \t]+", " "),
    ]
    .into_iter()
    .map(|(pattern, replacement)| (Regex::new(pattern).expect("the pattern is a valid regex"), replacement))
    .collect()
});

pub fn strip(text: &str) -> String {
    let mut cleaned = text.to_string();
    for (pattern, replacement) in INLINE_RULES.iter() {
        cleaned = pattern.replace_all(&cleaned, *replacement).into_owned();
    }
    cleaned.trim().to_string()
}
