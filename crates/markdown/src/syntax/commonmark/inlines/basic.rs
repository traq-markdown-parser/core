use super::entities::decode_entity;
use crate::{
    NodeKind, ParseError,
    engine::{
        Budget,
        inline::{InlineAction, InlineInput, InlineMatch},
    },
};

pub(super) fn escape(
    input: &InlineInput<'_>,
    budget: &mut Budget,
) -> Result<Option<InlineMatch>, ParseError> {
    let pos = input.position;
    let bytes = input.source.text.as_bytes();
    match bytes.get(pos + 1).copied() {
        Some(b'\n') => {
            let mut end = pos + 2;
            while matches!(bytes.get(end), Some(b' ' | b'\t')) {
                budget.spend(1)?;
                end += 1;
            }
            Ok(Some(InlineMatch::leaf(end, NodeKind::Hardbreak)))
        }
        Some(c) if c.is_ascii_punctuation() => Ok(Some(InlineMatch::leaf(
            pos + 2,
            NodeKind::text((c as char).to_string()),
        ))),
        _ => Ok(None),
    }
}

pub(super) fn newline(
    input: &InlineInput<'_>,
    budget: &mut Budget,
) -> Result<Option<InlineMatch>, ParseError> {
    let trim = input.trailing_text.len() - input.trailing_text.trim_end_matches(' ').len();
    let mut end = input.position + 1;
    while matches!(input.source.text.as_bytes().get(end), Some(b' ' | b'\t')) {
        budget.spend(1)?;
        end += 1;
    }
    Ok(Some(InlineMatch {
        end,
        action: InlineAction::TrimmedNode {
            trim,
            kind: if trim >= 2 {
                NodeKind::Hardbreak
            } else {
                NodeKind::Softbreak
            },
        },
    }))
}

pub(super) fn entity(
    input: &InlineInput<'_>,
    budget: &mut Budget,
) -> Result<Option<InlineMatch>, ParseError> {
    let Some((length, value)) = decode_entity(input.tail(), 7, 6) else {
        return Ok(None);
    };
    budget.spend(length)?;
    Ok(Some(InlineMatch::leaf(
        input.position + length,
        NodeKind::Text { value },
    )))
}

pub(super) fn code(
    input: &InlineInput<'_>,
    budget: &mut Budget,
) -> Result<Option<InlineMatch>, ParseError> {
    let pos = input.position;
    let source = &input.source.text;
    let bytes = source.as_bytes();
    let mut open_end = pos;
    while bytes.get(open_end) == Some(&b'`') {
        budget.spend(1)?;
        open_end += 1;
    }
    let mut end = open_end;
    // Searches are bounded by the shared work budget, including failed closers.
    while end < bytes.len() {
        budget.spend(1)?;
        if bytes[end] != b'`' {
            end += 1;
            continue;
        }
        let start = end;
        while bytes.get(end) == Some(&b'`') {
            budget.spend(1)?;
            end += 1;
        }
        if end - start == open_end - pos {
            let mut literal = source[open_end..start].replace('\n', " ");
            if literal.starts_with(' ')
                && literal.ends_with(' ')
                && literal.bytes().any(|b| b != b' ')
            {
                literal = literal[1..literal.len() - 1].into();
            }
            return Ok(Some(InlineMatch::leaf(
                end,
                NodeKind::InlineCode { literal },
            )));
        }
    }
    Ok(Some(InlineMatch::literal(open_end)))
}
