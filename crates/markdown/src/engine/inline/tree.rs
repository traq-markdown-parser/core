use super::token::{Token, TokenKind};
use crate::{
    Node, NodeKind, ParseError,
    engine::{Budget, source::SourceView},
};

pub(super) fn build(
    tokens: Vec<Token>,
    source: &SourceView,
    budget: &mut Budget,
) -> Result<Vec<Node>, ParseError> {
    let mut stack = vec![];
    let mut children = vec![];
    for token in tokens {
        budget.spend(1)?;
        let (kind, nested) = match token.kind {
            TokenKind::Empty => continue,
            TokenKind::Text(value) | TokenKind::Marker(value) => {
                if value.is_empty() {
                    continue;
                }
                (NodeKind::Text { value }, vec![])
            }
            TokenKind::Atom(kind, nested) => (kind, nested),
            TokenKind::Open(make, combined) => {
                budget.depth(stack.len() + 1)?;
                stack.push((make, combined, token.start, std::mem::take(&mut children)));
                continue;
            }
            TokenKind::Close => {
                let (make, combined, start, previous) =
                    stack.pop().ok_or(ParseError::InternalError)?;
                let nested = std::mem::replace(&mut children, previous);
                children.push(Node::new(
                    source.span(start, token.end),
                    make(combined),
                    nested,
                ));
                continue;
            }
        };
        if let NodeKind::Text { value } = &kind
            && let Some(Node {
                span,
                kind: NodeKind::Text { value: previous },
                ..
            }) = children.last_mut()
        {
            previous.push_str(value);
            span.end = source.span(token.start, token.end).end;
            continue;
        }
        children.push(Node::new(source.span(token.start, token.end), kind, nested));
    }
    if !stack.is_empty() {
        return Err(ParseError::InternalError);
    }
    Ok(children)
}
