fn is_boundary(characters: &[char], index: usize) -> bool {
    let current = characters[index];
    let Some(previous) = index.checked_sub(1).map(|position| characters[position]).filter(char::is_ascii_alphanumeric)
    else {
        return false;
    };
    let next = characters.get(index + 1).copied().unwrap_or(' ');
    let is_lower_to_upper = previous.is_ascii_lowercase() && current.is_ascii_uppercase();
    let is_digit_change = previous.is_ascii_digit() != current.is_ascii_digit();
    let is_acronym_end = previous.is_ascii_uppercase() && current.is_ascii_uppercase() && next.is_ascii_lowercase();
    is_lower_to_upper || is_digit_change || is_acronym_end
}

fn split_part(part: &str, words: &mut Vec<String>) {
    let characters: Vec<char> = part.chars().collect();
    let mut current = String::new();
    for (index, character) in characters.iter().enumerate() {
        let is_split = !character.is_ascii_alphanumeric() || is_boundary(&characters, index);
        if is_split && !current.is_empty() {
            words.push(std::mem::take(&mut current));
        }
        if character.is_ascii_alphanumeric() {
            current.push(*character);
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
}

pub fn split(name: &str) -> Vec<String> {
    let mut words = Vec::new();
    for part in name.trim_matches(['_', '$', '#']).split('_') {
        split_part(part, &mut words);
    }
    words
}
