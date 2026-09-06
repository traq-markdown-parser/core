use super::{
    apply::children_budget,
    scan::TextInput,
    token::{Token, TokenKind},
};
use crate::{
    NodeKind, ParseError,
    engine::{Budget, Grammar, source::SourceView},
};

pub(super) fn process(
    tokens: Vec<Token>,
    source: &SourceView,
    grammar: &Grammar,
    budget: &mut Budget,
) -> Result<Vec<Token>, ParseError> {
    let mut joined: Vec<Token> = vec![];
    for mut token in tokens {
        if let TokenKind::Marker(value) = token.kind {
            token.kind = TokenKind::Text(value);
        }
        if matches!(token.kind, TokenKind::Empty) {
            continue;
        }
        if let TokenKind::Text(value) = &token.kind
            && let Some(Token {
                end,
                kind: TokenKind::Text(previous),
                ..
            }) = joined.last_mut()
            && *end == token.start
        {
            previous.push_str(value);
            *end = token.end;
            continue;
        }
        joined.push(token);
    }
    for entry in &grammar.data.text {
        let mut result = vec![];
        let mut special = false;
        for token in joined {
            let TokenKind::Text(ref value) = token.kind else {
                special = matches!(token.kind, TokenKind::Atom(NodeKind::Text { .. }, _));
                result.push(token);
                continue;
            };
            budget.spend(value.len())?;
            let input = TextInput {
                source,
                range: token.start..token.end,
                after_decoded_text: special,
            };
            let matches = (entry.data.implementation.parse)(&input, budget)?;
            let mut begin = token.start;
            for found in matches {
                if found.start < begin
                    || found.start >= found.end
                    || found.end > token.end
                    || !source.text.is_char_boundary(found.start)
                    || !source.text.is_char_boundary(found.end)
                {
                    return Err(ParseError::InternalError);
                }
                if found.start > begin {
                    budget.token()?;
                    result.push(Token {
                        start: begin,
                        end: found.start,
                        kind: TokenKind::Text(source.text[begin..found.start].into()),
                    });
                }
                budget.token()?;
                children_budget(&found.children, budget)?;
                result.push(Token {
                    start: found.start,
                    end: found.end,
                    kind: TokenKind::Atom(found.kind, found.children),
                });
                begin = found.end;
            }
            if begin < token.end {
                result.push(Token {
                    start: begin,
                    end: token.end,
                    kind: TokenKind::Text(source.text[begin..token.end].into()),
                });
            }
            special = false;
        }
        joined = result;
    }
    Ok(joined)
}
