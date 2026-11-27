#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Python,
    Script,
    Go,
    Rust,
}

impl Language {
    pub fn from_profile(name: &str) -> Self {
        match name {
            "python" => Self::Python,
            "go" => Self::Go,
            "rust" => Self::Rust,
            _ => Self::Script,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Comment,
    Literal,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

const PYTHON_PREFIXES: &[u8] = b"rRbBuUfF";

#[derive(Clone, Copy)]
struct Quote {
    byte: u8,
    is_single_line: bool,
}
const RUST_ESCAPE_CHARS: usize = 12;

fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn find_line_end(bytes: &[u8], from: usize) -> usize {
    bytes[from..].iter().position(|byte| *byte == b'\n').map_or(bytes.len(), |offset| from + offset)
}

fn find_text_end(bytes: &[u8], from: usize, closer: &[u8]) -> usize {
    let mut index = from;
    while index < bytes.len() {
        if bytes[index] == b'\\' {
            index += 2;
        } else if bytes[index..].starts_with(closer) {
            return index + closer.len();
        } else {
            index += 1;
        }
    }
    bytes.len()
}

fn find_quote_end(bytes: &[u8], from: usize, quote: Quote) -> usize {
    let mut index = from;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 2,
            byte if byte == quote.byte => return index + 1,
            b'\n' if quote.is_single_line => return index,
            _ => index += 1,
        }
    }
    bytes.len()
}

fn find_python_start(bytes: &[u8], quote_index: usize) -> usize {
    let mut start = quote_index;
    while start > 0 && quote_index - start < 2 && PYTHON_PREFIXES.contains(&bytes[start - 1]) {
        start -= 1;
    }
    let is_in_identifier = start < quote_index && start > 0 && is_word(bytes[start - 1]);
    if is_in_identifier {
        quote_index
    } else {
        start
    }
}

fn find_python_token(bytes: &[u8], index: usize) -> Option<Token> {
    let quote = bytes[index];
    if quote == b'#' {
        return Some(Token { kind: TokenKind::Comment, start: index, end: find_line_end(bytes, index) });
    }
    if quote != b'"' && quote != b'\'' {
        return None;
    }
    let triple = [quote, quote, quote];
    let end = if bytes[index..].starts_with(&triple) {
        find_text_end(bytes, index + 3, &triple)
    } else {
        find_quote_end(bytes, index + 1, Quote { byte: quote, is_single_line: true })
    };
    Some(Token { kind: TokenKind::Literal, start: find_python_start(bytes, index), end })
}

fn find_raw_string(bytes: &[u8], index: usize) -> Option<Token> {
    let before = index.checked_sub(1).map(|position| bytes[position]);
    let is_byte_prefix = before == Some(b'b') && (index < 2 || !is_word(bytes[index - 2]));
    if before.is_some_and(is_word) && !is_byte_prefix {
        return None;
    }
    let hashes = bytes[index + 1..].iter().take_while(|byte| **byte == b'#').count();
    if bytes.get(index + 1 + hashes) != Some(&b'"') {
        return None;
    }
    let mut closer = vec![b'"'];
    closer.extend(std::iter::repeat_n(b'#', hashes));
    let from = index + 2 + hashes;
    let end = bytes[from..]
        .windows(closer.len())
        .position(|window| window == closer.as_slice())
        .map_or(bytes.len(), |offset| from + offset + closer.len());
    Some(Token { kind: TokenKind::Literal, start: index, end })
}

fn find_char_literal(text: &str, index: usize) -> Option<Token> {
    let rest = &text[index + 1..];
    if rest.starts_with('\\') {
        let close = rest.get(2..)?.char_indices().take(RUST_ESCAPE_CHARS).find(|(_, character)| *character == '\'')?;
        return Some(Token { kind: TokenKind::Literal, start: index, end: index + 3 + close.0 + 1 });
    }
    let character = rest.chars().next()?;
    let after = index + 1 + character.len_utf8();
    (text.as_bytes().get(after) == Some(&b'\'')).then_some(Token {
        kind: TokenKind::Literal,
        start: index,
        end: after + 1,
    })
}

fn find_literal(text: &str, index: usize, language: Language) -> Option<Token> {
    let bytes = text.as_bytes();
    let literal = |end: usize| Some(Token { kind: TokenKind::Literal, start: index, end });
    match (language, bytes[index]) {
        (Language::Rust, b'"') => {
            literal(find_quote_end(bytes, index + 1, Quote { byte: b'"', is_single_line: false }))
        }
        (_, b'"') => literal(find_quote_end(bytes, index + 1, Quote { byte: b'"', is_single_line: true })),
        (Language::Script, b'\'') | (Language::Go, b'\'') => {
            literal(find_quote_end(bytes, index + 1, Quote { byte: b'\'', is_single_line: true }))
        }
        (Language::Script, b'`') => {
            literal(find_quote_end(bytes, index + 1, Quote { byte: b'`', is_single_line: false }))
        }
        (Language::Go, b'`') => literal(find_line_end_of(bytes, index + 1, b'`')),
        (Language::Rust, b'\'') => find_char_literal(text, index),
        (Language::Rust, b'r') => find_raw_string(bytes, index),
        _ => None,
    }
}

fn find_line_end_of(bytes: &[u8], from: usize, closer: u8) -> usize {
    bytes[from..].iter().position(|byte| *byte == closer).map_or(bytes.len(), |offset| from + offset + 1)
}

fn find_c_token(text: &str, index: usize, language: Language) -> Option<Token> {
    let bytes = text.as_bytes();
    if bytes[index..].starts_with(b"//") {
        return Some(Token { kind: TokenKind::Comment, start: index, end: find_line_end(bytes, index) });
    }
    if bytes[index..].starts_with(b"/*") {
        let end = find_text_end(bytes, index + 2, b"*/");
        return Some(Token { kind: TokenKind::Comment, start: index, end });
    }
    find_literal(text, index, language)
}

pub fn list_tokens(text: &str, language: Language) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < text.len() {
        let token = if language == Language::Python {
            find_python_token(text.as_bytes(), index)
        } else {
            find_c_token(text, index, language)
        };
        match token {
            Some(found) => {
                index = found.end.max(index + 1);
                tokens.push(found);
            }
            None => index += 1,
        }
    }
    tokens
}
