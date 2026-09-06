use super::markers::*;
use crate::syntax::commonmark::references;
use crate::{
    ParseError,
    engine::{
        Budget,
        block::{BlockInput, BlockMatch, Definition, Interrupt},
    },
};

pub(super) fn parse(
    input: &BlockInput<'_>,
    budget: &mut Budget,
    normalize: fn(&str) -> Option<String>,
) -> Result<Option<BlockMatch>, ParseError> {
    let line = input.current();
    if line.starts_with("    ") || !line.trim_start_matches([' ', '\t']).starts_with('[') {
        return Ok(None);
    }
    let first = input.start;
    let lines = input.lines;
    let mut last = first + 1;
    let mut found = None;
    while last <= lines.len() {
        if last > first + 1 && input.interrupts(last - 1, Interrupt::Reference) {
            break;
        }
        let text = input
            .source
            .literal(lines[first].start..lines[last - 1].end);
        budget.spend(text.len())?;
        if let Some(definition) = references::definition(&text, budget, normalize)? {
            found = Some((last, definition));
            if last == lines.len()
                || !input
                    .line(last)
                    .trim_start_matches([' ', '\t'])
                    .starts_with(['\'', '"', '('])
            {
                break;
            }
        }
        if last == first + 1
            && references::label_end(line.trim_start_matches([' ', '\t'])).is_some()
            && !line.contains("]:")
        {
            break;
        }
        last += 1;
    }
    let Some((mut end, (key, destination, title))) = found else {
        return Ok(None);
    };
    if end < lines.len() && blank(input.line(end)) {
        end += 1;
    }
    let mut result = BlockMatch::ignore(end);
    result.definitions.push(Definition {
        key,
        destination,
        title,
    });
    Ok(Some(result))
}
