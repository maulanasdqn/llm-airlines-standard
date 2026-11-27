use super::lexer::{Language, Token, TokenKind};

pub fn build_code_text(text: &str, tokens: &[Token]) -> String {
    let mut bytes = text.as_bytes().to_vec();
    for token in tokens {
        for byte in &mut bytes[token.start..token.end.min(text.len())] {
            if *byte != b'\n' {
                *byte = b' ';
            }
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

pub fn build_line_index(text: &str) -> Vec<usize> {
    std::iter::once(0).chain(text.match_indices('\n').map(|(offset, _)| offset + 1)).collect()
}

pub fn find_line(index: &[usize], offset: usize) -> usize {
    index.partition_point(|start| *start <= offset)
}

pub fn compute_depths(code_text: &str) -> Vec<usize> {
    let mut depths = Vec::with_capacity(code_text.len() + 1);
    let mut depth: usize = 0;
    for byte in code_text.bytes() {
        depths.push(depth);
        match byte {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    depths.push(depth);
    depths
}

pub fn to_comment_body(text: &str) -> &str {
    if let Some(body) = text.strip_prefix('#') {
        return body;
    }
    if let Some(body) = text.strip_prefix("//") {
        return body;
    }
    text.get(2..text.len().saturating_sub(2)).unwrap_or_default()
}

pub fn is_doc_comment(text: &str, language: Language) -> bool {
    let prefixes: &[&str] = match language {
        Language::Script => &["/**"],
        Language::Rust => &["///", "//!", "/**", "/*!"],
        _ => &[],
    };
    prefixes.iter().any(|prefix| text.starts_with(prefix)) && !text.starts_with("/**/") && !text.starts_with("////")
}

fn is_bare_statement(text: &str, code_text: &str, token: &Token) -> bool {
    let line_start = text[..token.start].rfind('\n').map_or(0, |index| index + 1);
    let line_end = text[token.end.min(text.len())..].find('\n').map_or(text.len(), |offset| token.end + offset);
    let suffix = code_text.get(token.end..line_end).unwrap_or_default().trim();
    text[line_start..token.start].trim().is_empty() && (suffix.is_empty() || suffix == ";")
}

pub fn find_docstrings(text: &str, tokens: &[Token], code_text: &str) -> Vec<Token> {
    let depths = compute_depths(code_text);
    tokens
        .iter()
        .filter(|token| token.kind == TokenKind::Literal)
        .filter(|token| {
            depths.get(token.start).copied().unwrap_or_default() == 0 && is_bare_statement(text, code_text, token)
        })
        .copied()
        .collect()
}
