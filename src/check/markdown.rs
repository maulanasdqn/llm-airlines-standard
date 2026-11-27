use std::collections::BTreeSet;
use std::sync::LazyLock;

use regex::Regex;

use super::finding::{is_file_ignored, parse_ignore_ids, Suppressions};

static FENCE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*(```|~~~)").expect("the pattern is a valid regex"));
static LIST_ITEM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?:[-*+]|\d+[.)])\s+(.*)$").expect("the pattern is a valid regex"));
static ORDERED_ITEM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*\d+[.)]\s").expect("the pattern is a valid regex"));
static ALERT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^>\s*\[!(\w+)\]\s*$").expect("the pattern is a valid regex"));
static HEADING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s{0,3}#{1,6}\s").expect("the pattern is a valid regex"));
static INLINE_COMMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<!--(.*?)-->").expect("the pattern is a valid regex"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    Paragraph,
    Item,
    Alert,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub kind: BlockKind,
    pub lines: Vec<(usize, String)>,
    pub is_step: bool,
    pub marker: String,
    ignored: BTreeSet<String>,
}

#[derive(Debug, Default)]
pub struct State {
    pub blocks: Vec<Block>,
    pub suppressions: Suppressions,
    pub is_ignored: bool,
    current: Option<Block>,
    fence: String,
    is_in_comment: bool,
    pending_ignore: BTreeSet<String>,
}

fn build_block(kind: BlockKind, marker: &str, is_step: bool) -> Block {
    Block { kind, lines: Vec::new(), is_step, marker: marker.to_string(), ignored: BTreeSet::new() }
}

fn find_content_start(lines: &[&str]) -> usize {
    if lines.first().map(|line| line.trim()) != Some("---") {
        return 0;
    }
    lines.iter().skip(1).position(|line| line.trim() == "---").map_or(0, |index| index + 2)
}

fn add_suppression(state: &mut State, line: usize, rules: &BTreeSet<String>) {
    if !rules.is_empty() {
        state.suppressions.entry(line).or_default().extend(rules.iter().cloned());
    }
}

fn close_block(state: &mut State) {
    let Some(block) = state.current.take().filter(|block| !block.lines.is_empty()) else {
        return;
    };
    let numbers: Vec<usize> = block.lines.iter().map(|(number, _)| *number).collect();
    let ignored = block.ignored.clone();
    state.blocks.push(block);
    for number in numbers {
        add_suppression(state, number, &ignored);
    }
}

fn start_block(state: &mut State, mut block: Block) {
    close_block(state);
    block.ignored = std::mem::take(&mut state.pending_ignore);
    state.current = Some(block);
}

fn add_line(state: &mut State, number: usize, text: &str) {
    if let Some(block) = state.current.as_mut() {
        block.lines.push((number, text.to_string()));
    }
}

fn read_comment_line(state: &mut State, line: &str) {
    state.pending_ignore.extend(parse_ignore_ids(line));
    state.is_ignored = state.is_ignored || is_file_ignored(line);
    state.is_in_comment = !line.contains("-->");
}

fn read_quote_line(state: &mut State, number: usize, line: &str) {
    if let Some(alert) = ALERT.captures(line.trim()) {
        start_block(state, build_block(BlockKind::Alert, &alert[1].to_uppercase(), false));
        return;
    }
    if state.current.as_ref().is_some_and(|block| block.kind == BlockKind::Alert) {
        add_line(state, number, line.trim_start()[1..].trim());
        return;
    }
    close_block(state);
}

fn read_text_line(state: &mut State, number: usize, line: &str) {
    for comment in INLINE_COMMENT.captures_iter(line) {
        let rules = parse_ignore_ids(&comment[1]);
        add_suppression(state, number, &rules);
    }
    if let Some(item) = LIST_ITEM.captures(line) {
        start_block(state, build_block(BlockKind::Item, "", ORDERED_ITEM.is_match(line)));
        add_line(state, number, &item[1]);
        return;
    }
    if state.current.as_ref().is_none_or(|block| block.kind == BlockKind::Alert) {
        start_block(state, build_block(BlockKind::Paragraph, "", false));
    }
    add_line(state, number, line.trim());
}

fn read_open_line(state: &mut State, number: usize, line: &str) {
    let stripped = line.trim_start();
    if let Some(fence) = FENCE.captures(line) {
        close_block(state);
        state.fence = fence[1].to_string();
    } else if stripped.starts_with("<!--") {
        close_block(state);
        read_comment_line(state, line);
    } else if stripped.starts_with('>') {
        read_quote_line(state, number, line);
    } else if line.trim().is_empty() || HEADING.is_match(line) || stripped.starts_with(['|', '<']) {
        close_block(state);
    } else {
        read_text_line(state, number, line);
    }
}

fn read_line(state: &mut State, number: usize, line: &str) {
    if !state.fence.is_empty() {
        if line.trim().starts_with(state.fence.as_str()) {
            state.fence.clear();
        }
        return;
    }
    if state.is_in_comment {
        read_comment_line(state, line);
        return;
    }
    read_open_line(state, number, line);
}

pub fn parse(text: &str) -> State {
    let mut state = State::default();
    let lines: Vec<&str> = text.lines().collect();
    let first = find_content_start(&lines);
    for (index, line) in lines.iter().enumerate().skip(first) {
        read_line(&mut state, index + 1, line);
    }
    close_block(&mut state);
    state
}
