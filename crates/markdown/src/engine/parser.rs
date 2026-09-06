use super::{Budget, Grammar, Limits, ParseError, block, inline, source::SourceView};
use crate::ast::{Document, NodeKind};

pub struct Parser {
    grammar: Grammar,
    limits: Limits,
}
impl Parser {
    pub fn new(grammar: &Grammar) -> Self {
        Self {
            grammar: grammar.clone(),
            limits: Limits::default(),
        }
    }
    pub fn with_limits(mut self, limits: Limits) -> Self {
        self.limits = limits;
        self
    }
    pub fn parse(&self, source: &str) -> Result<Document, ParseError> {
        self.run(source, false)
    }
    pub fn parse_inline(&self, source: &str) -> Result<Document, ParseError> {
        self.run(source, true)
    }
    fn run(&self, source: &str, inline_only: bool) -> Result<Document, ParseError> {
        let grammar = &self.grammar;
        if source.len() > self.limits.input_bytes {
            return Err(ParseError::limit("input_bytes"));
        }
        let view = SourceView::new(source);
        let mut budget = Budget::new(self.limits);
        let children = if inline_only {
            inline::parse(&view, grammar, &mut budget, &block::References::new())?
        } else {
            block::parse(&view, grammar, &mut budget)?
        };
        let mut pending: Vec<_> = children
            .iter()
            .map(|node| (node, 1, 0..source.len()))
            .collect();
        let mut count = 0;
        while let Some((node, depth, parent)) = pending.pop() {
            count += 1;
            budget.spend(1)?;
            budget.depth(depth)?;
            if count > self.limits.nodes {
                return Err(ParseError::limit("tokens"));
            }
            let span = node.span;
            if span.start > span.end
                || span.start < parent.start
                || span.end > parent.end
                || !source.is_char_boundary(span.start)
                || !source.is_char_boundary(span.end)
            {
                return Err(ParseError::InternalError);
            }
            if !node.kind.valid_shape(!node.children.is_empty()) {
                return Err(ParseError::InternalError);
            }
            if let NodeKind::Extension { name, data } = &node.kind {
                let index = grammar
                    .data
                    .extension_index
                    .get(name.as_str())
                    .ok_or(ParseError::InternalError)?;
                let definition = &grammar.data.extensions[*index];
                if !(definition.validate)(data) {
                    return Err(ParseError::InternalError);
                }
            }
            pending.extend(
                node.children
                    .iter()
                    .map(|child| (child, depth + 1, span.start..span.end)),
            );
        }
        Ok(Document {
            source: source.into(),
            children,
        })
    }
}
