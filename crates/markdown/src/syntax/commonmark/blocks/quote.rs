use super::{markers::*, paragraph::paragraph_start};
use crate::{
    NodeKind, ParseError,
    engine::{
        Budget,
        block::{BlockInput, BlockMatch, DraftNode, Interrupt},
    },
};

pub fn parse(input: &BlockInput<'_>, _: &mut Budget) -> Result<Option<BlockMatch>, ParseError> {
    if quote(input.current()).is_none() {
        return Ok(None);
    }
    let source = input.source;
    let lines = input.lines;
    let first = input.start;
    let mut end = first;
    let mut ranges = vec![];
    let mut lazy_lines = vec![];
    let mut lazy = false;
    while end < lines.len() {
        let next = input.line(end);
        let indent = next.len() - next.trim_start_matches(' ').len();
        if let Some(prefix) = quote(&next[indent..]).map(|width| width + indent) {
            lazy = paragraph_start(&next[prefix..], input);
            ranges.push(lines[end].start + prefix..lines[end].end);
        } else if lazy && !input.interrupts(end, Interrupt::LazyContinuation) && !blank(next) {
            lazy_lines.push(source.span(lines[end].start, lines[end].start).start);
            ranges.push(lines[end].clone());
        } else {
            break;
        }
        end += 1;
    }
    let mut view = source.select(&ranges);
    for line in lazy_lines {
        view.mark_lazy(line);
    }
    Ok(Some(BlockMatch::node(
        end,
        DraftNode::blocks(
            source.span(lines[first].start, lines[end - 1].end),
            NodeKind::Blockquote,
            view,
        ),
    )))
}
