use super::lexer::Language;
use super::names::Name;

pub fn count_code_lines(text: &str) -> usize {
    text.lines().filter(|line| !line.trim().is_empty()).count()
}

fn find_closing(bytes: &[u8], start: usize, pair: (u8, u8)) -> Option<usize> {
    let mut depth: usize = 0;
    for (position, byte) in bytes.iter().enumerate().skip(start) {
        if *byte == pair.0 {
            depth += 1;
        } else if *byte == pair.1 {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(position);
            }
        }
    }
    None
}

fn find_generics_end(bytes: &[u8], position: usize) -> usize {
    let start = position + bytes[position..].iter().take_while(|byte| **byte == b' ' || **byte == b'\t').count();
    if bytes.get(start) != Some(&b'<') {
        return position;
    }
    find_closing(bytes, start, (b'<', b'>')).map_or(position, |closing| closing + 1)
}

fn find_body_start(bytes: &[u8], name: &Name) -> Option<usize> {
    let position = find_generics_end(bytes, name.offset + name.name.len());
    let parameters = position + bytes[position..].iter().position(|byte| *byte == b'(')?;
    let closing = find_closing(bytes, parameters, (b'(', b')'))?;
    let body = closing + bytes[closing..].iter().position(|byte| *byte == b'{' || *byte == b';')?;
    (bytes[body] == b'{').then_some(body)
}

fn measure_braced(code_text: &str, name: &Name) -> usize {
    let bytes = code_text.as_bytes();
    let Some(closing) = find_body_start(bytes, name).and_then(|body| find_closing(bytes, body, (b'{', b'}'))) else {
        return 0;
    };
    let line_start = code_text[..name.offset].rfind('\n').map_or(0, |index| index + 1);
    count_code_lines(&code_text[line_start..=closing])
}

fn measure_indented(code_text: &str, name: &Name) -> usize {
    let line_start = code_text[..name.offset].rfind('\n').map_or(0, |index| index + 1);
    let indent = name.offset - line_start - code_text[line_start..name.offset].trim_start().len();
    let mut length = 1;
    for line in code_text[line_start..].lines().skip(1).filter(|line| !line.trim().is_empty()) {
        let line_indent = line.len() - line.trim_start().len();
        if line_indent <= indent && !line.trim_start().starts_with(')') {
            break;
        }
        length += 1;
    }
    length
}

pub fn measure_function(code_text: &str, name: &Name, language: Language) -> usize {
    if language == Language::Python {
        return measure_indented(code_text, name);
    }
    measure_braced(code_text, name)
}
