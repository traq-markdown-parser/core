use super::markers::*;
use crate::{
    NodeKind, ParseError,
    engine::{
        Budget,
        block::{BlockInput, BlockMatch, DraftNode, Interrupt},
    },
};

fn setext(input: &BlockInput<'_>, line: usize) -> Option<u8> {
    if super::super::context::is_lazy(
        input.source,
        input
            .source
            .span(input.lines[line].start, input.lines[line].start)
            .start,
    ) {
        None
    } else {
        underline(input.line(line))
    }
}
pub(super) fn paragraph_start(mut line: &str, input: &BlockInput<'_>) -> bool {
    loop {
        if blank(line) || line.starts_with("    ") || line.starts_with('\t') {
            return false;
        }
        if let Some(prefix) = quote(line) {
            line = &line[prefix..];
        } else if let Some(marker) = item(line) {
            line = &line[marker.content..];
        } else {
            return !input.interrupts_text(line, None, Interrupt::LazyContinuation);
        }
    }
}

pub(super) fn parse(
    input: &BlockInput<'_>,
    _: &mut Budget,
) -> Result<Option<BlockMatch>, ParseError> {
    let source = input.source;
    let lines = input.lines;
    let first = input.start;
    let mut end = first + 1;
    while end < lines.len()
        && setext(input, end).is_none()
        && !input.interrupts(end, Interrupt::Paragraph)
    {
        end += 1;
    }
    let raw = &source.text[lines[first].start..lines[end - 1].end];
    let start = lines[first].start + raw.len() - raw.trim_start_matches(ascii_space).len();
    let finish = lines[end - 1].start + input.line(end - 1).trim_end_matches(ascii_space).len();
    let view = source.select(&[start..finish.max(start)]);
    let kind = if let Some(level) = (end < lines.len()).then(|| setext(input, end)).flatten() {
        end += 1;
        NodeKind::Heading { level }
    } else {
        NodeKind::Paragraph
    };
    Ok(Some(BlockMatch::node(
        end,
        DraftNode::inline(
            source.span(lines[first].start, lines[end - 1].end),
            kind,
            view,
        ),
    )))
}
