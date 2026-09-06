use super::{Alignment, CellData, RowData, TableData, cells};
use crate::{
    ParseError,
    ast::ExtensionData,
    engine::{
        Budget,
        block::{BlockInput, BlockMatch, DraftContent, DraftNode, Interrupt},
    },
};

pub(super) fn parse(
    input: &BlockInput<'_>,
    budget: &mut Budget,
) -> Result<Option<BlockMatch>, ParseError> {
    let first = input.start;
    let Some(align) = (first + 1 < input.lines.len())
        .then(|| cells::alignment(input.current(), input.line(first + 1)))
        .flatten()
    else {
        return Ok(None);
    };
    let mut children = vec![row(input, first, &align, true, budget)?];
    let mut end = first + 2;
    while end < input.lines.len()
        && !input.interrupts(end, Interrupt::BlockBody)
        && !input.line(end).starts_with("    ")
    {
        children.push(row(input, end, &align, false, budget)?);
        end += 1;
    }
    Ok(Some(input.matched(
        end,
        TableData {}.node_kind()?,
        DraftContent::Nodes(children),
    )?))
}

fn row(
    input: &BlockInput<'_>,
    index: usize,
    align: &[Option<Alignment>],
    header: bool,
    budget: &mut Budget,
) -> Result<DraftNode, ParseError> {
    let source = input.source;
    let line = &input.lines[index];
    let columns = cells::cells(input.line(index), line.start);
    let mut children = vec![];
    budget.token()?;
    for (column, alignment) in align.iter().enumerate() {
        budget.token()?;
        let range = columns.get(column).cloned().unwrap_or(line.end..line.end);
        let raw = &source.text[range.clone()];
        let start = range.start + raw.len() - raw.trim_start().len();
        let end = (range.start + raw.trim_end().len()).max(start);
        let mut pieces = vec![];
        let mut begin = start;
        for pos in start..end {
            if source.text.as_bytes()[pos] == b'|'
                && pos > start
                && source.text.as_bytes()[pos - 1] == b'\\'
            {
                pieces.push(begin..pos - 1);
                begin = pos;
            }
        }
        pieces.push(begin..end);
        let data = CellData {
            alignment: *alignment,
        };
        children.push(DraftNode::inline(
            source.span(start, end),
            data.node_kind()?,
            source.select(&pieces),
        ));
    }
    Ok(DraftNode::nodes(
        source.span(line.start, line.end),
        RowData { header }.node_kind()?,
        children,
    ))
}
