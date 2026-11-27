use super::lexer::{is_word, Token, TokenKind};

const VALUE_KEYWORDS: [&str; 14] = [
    "return",
    "typeof",
    "instanceof",
    "in",
    "of",
    "new",
    "delete",
    "void",
    "throw",
    "case",
    "do",
    "else",
    "yield",
    "await",
];
const VALUE_OPERATORS: &[u8] = b"(,=:[!&|?{};+-*%^~/";
const ARROW: &[u8] = b"=>";

fn find_word_start(bytes: &[u8], end: usize) -> usize {
    bytes[..end].iter().rposition(|byte| !is_word(*byte)).map_or(0, |position| position + 1)
}

fn is_value_expected(bytes: &[u8], index: usize) -> bool {
    let Some(last) = bytes[..index].iter().rposition(|byte| !byte.is_ascii_whitespace()) else {
        return true;
    };
    match bytes[last] {
        byte if is_word(byte) => {
            let word = &bytes[find_word_start(bytes, last)..=last];
            VALUE_KEYWORDS.iter().any(|keyword| keyword.as_bytes() == word)
        }
        b'>' => bytes[..=last].ends_with(ARROW),
        byte => VALUE_OPERATORS.contains(&byte),
    }
}

fn find_body_end(bytes: &[u8], from: usize) -> Option<usize> {
    let mut index = from;
    let mut is_in_class = false;
    while let Some(&byte) = bytes.get(index) {
        match (byte, is_in_class) {
            (b'\\', _) => index += 1,
            (b'\n', _) => return None,
            (b'[', false) => is_in_class = true,
            (b']', true) => is_in_class = false,
            (b'/', false) => return Some(index + 1),
            _ => {}
        }
        index += 1;
    }
    None
}

pub fn find(text: &str, index: usize) -> Option<Token> {
    let bytes = text.as_bytes();
    if !is_value_expected(bytes, index) {
        return None;
    }
    let body_end = find_body_end(bytes, index + 1)?;
    let end = body_end + bytes[body_end..].iter().take_while(|byte| is_word(**byte)).count();
    Some(Token { kind: TokenKind::Literal, start: index, end })
}
