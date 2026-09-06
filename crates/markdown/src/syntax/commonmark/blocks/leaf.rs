use super::markers::*;
use crate::{
    NodeKind, ParseError,
    engine::{
        Budget,
        block::{BlockInput, BlockMatch, DraftNode},
    },
};

pub(super) fn fenced(
    input: &BlockInput<'_>,
    _: &mut Budget,
) -> Result<Option<BlockMatch>, ParseError> {
    let Some((marker, count, info)) = fence(input.current()) else {
        return Ok(None);
    };
    let source = input.source;
    let lines = input.lines;
    let first = input.start;
    let info_end = lines[first].start + input.current().trim_end().len();
    let info = source.literal(info_end - info.len()..info_end).into_owned();
    let indent = input.current().len() - input.current().trim_start_matches(' ').len();
    let mut end = first + 1;
    while end < lines.len() {
        if fence(input.line(end))
            .is_some_and(|(m, n, info)| m == marker && n >= count && info.is_empty())
        {
            break;
        }
        end += 1;
    }
    let literal = lines[first + 1..end]
        .iter()
        .map(|line| {
            let raw = &source.text[line.clone()];
            let remove = indent.min(raw.len() - raw.trim_start_matches(' ').len());
            source.literal(line.start + remove..line.end)
        })
        .collect();
    if end < lines.len() {
        end += 1;
    }
    Ok(Some(BlockMatch::node(
        end,
        DraftNode::leaf(
            source.span(lines[first].start, lines[end - 1].end),
            NodeKind::CodeBlock {
                fenced: true,
                info,
                literal,
            },
        ),
    )))
}

pub(super) fn indented(
    input: &BlockInput<'_>,
    _: &mut Budget,
) -> Result<Option<BlockMatch>, ParseError> {
    if !input.current().starts_with("    ") {
        return Ok(None);
    }
    let source = input.source;
    let lines = input.lines;
    let first = input.start;
    let mut end = first;
    let mut literal = String::new();
    while end < lines.len() {
        let raw = &source.text[lines[end].clone()];
        if blank(input.line(end)) {
            let next = (end + 1..lines.len()).find(|i| !blank(input.line(*i)));
            if next.is_some_and(|i| input.line(i).starts_with("    ")) {
                let remove = 4.min(raw.len() - raw.trim_start_matches(' ').len());
                literal.push_str(&source.literal(lines[end].start + remove..lines[end].end));
                end += 1;
                continue;
            }
            break;
        } else if raw.starts_with("    ") {
            literal.push_str(&source.literal(lines[end].start + 4..lines[end].end));
            end += 1;
        } else {
            break;
        }
    }
    if !literal.ends_with('\n') {
        literal.push('\n');
    }
    Ok(Some(BlockMatch::node(
        end,
        DraftNode::leaf(
            source.span(lines[first].start, lines[end - 1].end),
            NodeKind::CodeBlock {
                fenced: false,
                info: String::new(),
                literal,
            },
        ),
    )))
}

pub(super) fn atx(
    input: &BlockInput<'_>,
    _: &mut Budget,
) -> Result<Option<BlockMatch>, ParseError> {
    let Some((level, prefix)) = heading(input.current()) else {
        return Ok(None);
    };
    let source = input.source;
    let line = &input.lines[input.start];
    let mut end = line.start + input.current().trim_end().len().max(prefix);
    let text = &source.text[line.start + prefix..end];
    let without = text.trim_end_matches('#');
    if without.is_empty() || without.ends_with([' ', '\t']) {
        end = line.start + prefix + without.trim_end().len();
    }
    let view = source.select(&[line.start + prefix..end]);
    Ok(Some(BlockMatch::node(
        input.start + 1,
        DraftNode::inline(
            source.span(line.start, line.end),
            NodeKind::Heading { level },
            view,
        ),
    )))
}

pub(super) fn thematic_break(
    input: &BlockInput<'_>,
    _: &mut Budget,
) -> Result<Option<BlockMatch>, ParseError> {
    if !thematic(input.current()) {
        return Ok(None);
    }
    let line = &input.lines[input.start];
    let marker = input
        .current()
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace())
        .collect();
    Ok(Some(BlockMatch::node(
        input.start + 1,
        DraftNode::leaf(
            input.source.span(line.start, line.end),
            NodeKind::ThematicBreak { marker },
        ),
    )))
}
