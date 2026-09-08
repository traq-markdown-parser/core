use super::BlockMathData;
use markdown_parser::{
    ParseError,
    engine::{
        Budget,
        block::{BlockInput, BlockMatch},
    },
};

pub(super) fn parse(
    input: &BlockInput<'_>,
    _: &mut Budget,
) -> Result<Option<BlockMatch>, ParseError> {
    let Some(rest) = input.current().strip_prefix("$$") else {
        return Ok(None);
    };
    let source = input.source;
    let lines = input.lines;
    let first = input.start;
    let mut end = first + 1;
    let mut tex = String::new();
    if let Some(inner) = rest.strip_suffix("$$") {
        tex.push_str(&source.literal(lines[first].start + 2..lines[first].start + 2 + inner.len()));
    } else {
        tex.push_str(&source.literal(lines[first].start + 2..lines[first].start + 2 + rest.len()));
        tex.push('\n');
        while end < lines.len() {
            let next = input.line(end);
            let start = lines[end].start;
            end += 1;
            if let Some(inner) = next.trim_end().strip_suffix("$$") {
                tex.push_str(&source.literal(start..start + inner.len()));
                break;
            }
            tex.push_str(&source.literal(start..start + next.len()));
            tex.push('\n');
        }
    }
    Ok(Some(input.leaf(
        end,
        markdown_parser::NodeKind::new(BlockMathData { tex }),
    )?))
}
