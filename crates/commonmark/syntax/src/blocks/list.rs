use super::markers::*;
use markdown_parser::{
    NodeKind, ParseError,
    engine::{
        Budget,
        block::{BlockInput, BlockMatch, DraftNode, Interrupt},
    },
};

pub(super) fn parse(
    input: &BlockInput<'_>,
    budget: &mut Budget,
) -> Result<Option<BlockMatch>, ParseError> {
    let Some(first_marker) = item(input.current()) else {
        return Ok(None);
    };
    let source = input.source;
    let lines = input.lines;
    let first = input.start;
    let mut i = first;
    let mut children = vec![];
    let mut tight = true;
    while i < lines.len() {
        if thematic(content(source, &lines[i])) {
            break;
        }
        let Some(marker) = item(content(source, &lines[i])) else {
            break;
        };
        if first_marker.ordered != marker.ordered || first_marker.delimiter != marker.delimiter {
            break;
        }
        let item_start = i;
        let first_content = &content(source, &lines[i])[marker.content..];
        let lazy = super::paragraph::paragraph_start(first_content, input);
        let end = if first_content.is_empty()
            && (i + 1 == lines.len() || blank(content(source, &lines[i + 1])))
        {
            lines[i].start + marker.content
        } else {
            lines[i].end
        };
        let mut ranges = vec![lines[i].start + marker.content..end];
        i += 1;
        while i < lines.len() {
            let continuation = content(source, &lines[i]);
            if blank(continuation) {
                let next = (i + 1..lines.len()).find(|j| !blank(content(source, &lines[*j])));
                let Some(next) = next else {
                    if !blank(first_content) {
                        ranges.extend_from_slice(&lines[i..]);
                    }
                    i = lines.len();
                    break;
                };
                let next_line = content(source, &lines[next]);
                if blank(first_content) {
                    tight &= !item(next_line).is_some_and(|m| {
                        m.ordered == first_marker.ordered && m.delimiter == first_marker.delimiter
                    });
                    i = next;
                    break;
                }
                let next_indent = next_line.len() - next_line.trim_start_matches(' ').len();
                if next_indent >= marker.width {
                    ranges.push(lines[i].clone());
                    i += 1;
                    continue;
                }
                if item(next_line).is_some_and(|m| {
                    m.ordered == first_marker.ordered && m.delimiter == first_marker.delimiter
                }) {
                    tight = false;
                    ranges.extend_from_slice(&lines[i..next]);
                    i = next;
                } else {
                    if !blank(first_content) {
                        ranges.extend_from_slice(&lines[i..next]);
                    }
                    i = next;
                }
                if blank(first_content) {
                    i = next;
                }
                break;
            }
            let indent = continuation.len() - continuation.trim_start_matches(' ').len();
            if indent >= marker.width {
                ranges.push(lines[i].start + marker.width..lines[i].end);
            } else if lazy
                && !input.interrupts_text(continuation, None, Interrupt::LazyContinuation)
                && item(continuation).is_none()
            {
                ranges.push(lines[i].clone());
            } else {
                break;
            }
            i += 1;
        }
        // Blank continuation lines still lose the list's indentation,
        // including when they become literal content of a fenced block.
        for range in ranges.iter_mut().skip(1) {
            let raw = &source.text()[range.clone()];
            if blank(raw.trim_end_matches('\n')) {
                range.start += marker
                    .width
                    .min(raw.len() - raw.trim_start_matches(' ').len());
            }
        }
        budget.token()?;
        children.push(DraftNode::blocks(
            source.span_for(lines[item_start].start..lines[i - 1].end)?,
            NodeKind::new(markdown_commonmark_contracts::ListItem {
                marker: marker.text,
            }),
            source.join(&ranges)?,
        ));
    }
    let mut node = DraftNode::nodes(
        source.span_for(lines[first].start..lines[i - 1].end)?,
        NodeKind::new(markdown_commonmark_contracts::List {
            ordered: first_marker.ordered,
            start: first_marker.start,
            tight,
        }),
        children,
    );
    node.finish = Some(finish_list);
    Ok(Some(BlockMatch::node(i, node)))
}
fn finish_list(node: &mut DraftNode) {
    let loose = node.children().iter().any(|item| {
        item.is_loose()
            || item
                .children()
                .iter()
                .filter(|child| {
                    child
                        .kind
                        .get::<markdown_commonmark_contracts::Paragraph>()
                        .is_some()
                })
                .count()
                > 1
    });
    if let Some(list) = node.kind.get_mut::<markdown_commonmark_contracts::List>() {
        list.tight &= !loose;
    }
}
