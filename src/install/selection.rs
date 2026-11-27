use std::io::{BufRead, Write};

use crate::error::{Error, Result};

const ALL_WORDS: [&str; 3] = ["all", "a", "*"];
const YES_WORDS: [&str; 2] = ["y", "yes"];

pub struct Menu<'a> {
    pub question: &'a str,
    pub options: &'a [String],
    pub default: &'a [usize],
}

pub struct Console {
    pub input: Box<dyn BufRead>,
}

pub fn parse(text: &str, count: usize, default: &[usize]) -> Result<Vec<usize>> {
    let lowered = text.trim().to_lowercase();
    let words: Vec<&str> = lowered
        .split(|character: char| character.is_whitespace() || character == ',')
        .filter(|word| !word.is_empty())
        .collect();
    if words.is_empty() {
        return Ok(default.to_vec());
    }
    if words.iter().any(|word| ALL_WORDS.contains(word)) {
        return Ok((0..count).collect());
    }
    let mut indexes = Vec::new();
    for word in words {
        let number = word.parse::<usize>().ok().filter(|number| (1..=count).contains(number)).ok_or_else(|| {
            Error::new(format!(
                "\"{word}\" is not a number from 1 to {count}. Type numbers such as \"1 3\", or \"all\"."
            ))
        })?;
        if !indexes.contains(&(number - 1)) {
            indexes.push(number - 1);
        }
    }
    Ok(indexes)
}

pub fn read_answer(console: &mut Console, prompt: &str) -> Result<String> {
    print!("{prompt}");
    std::io::stdout().flush()?;
    let mut answer = String::new();
    let count = console.input.read_line(&mut answer)?;
    if count == 0 {
        return Err(Error::new(
            "The installer got no answer. Use --skills and --target or --dir to install with no questions.",
        ));
    }
    Ok(answer.trim_end_matches(['\n', '\r']).to_string())
}

pub fn ask_choice(console: &mut Console, menu: &Menu) -> Result<Vec<usize>> {
    println!("{}", menu.question);
    for (number, option) in menu.options.iter().enumerate() {
        println!("  {}) {option}", number + 1);
    }
    let default_text = menu.default.iter().map(|index| (index + 1).to_string()).collect::<Vec<String>>().join(", ");
    loop {
        let answer = read_answer(console, &format!("Type numbers, or press Enter for {default_text}: "))?;
        match parse(&answer, menu.options.len(), menu.default) {
            Ok(indexes) => return Ok(indexes),
            Err(error) => println!("{error}"),
        }
    }
}

pub fn ask_confirmation(console: &mut Console, question: &str) -> Result<bool> {
    let answer = read_answer(console, &format!("{question} [y/N]: "))?;
    Ok(YES_WORDS.contains(&answer.trim().to_lowercase().as_str()))
}
